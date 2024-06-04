use std::time::Duration;

use bevy::{core::Zeroable, prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    components::{
        Asteroid, Collidable, Firing, FiringTimer, Position, RotateSpeed, Rotation, Ship, Size,
        Thrust, UfoBullet, Velocity,
    },
    events::ExplosionEvent,
    resources::WorldSize,
};

const NORMAL_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(389743489572398);
const THRUSTING_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(38475109234891725);

pub fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ship_query: Query<&Ship>,
    asset_server: Res<AssetServer>,
) {
    if !ship_query.is_empty() {
        return;
    }

    materials.insert(NORMAL_SHIP_COLOR_ID, Color::ANTIQUE_WHITE.into());
    materials.insert(THRUSTING_SHIP_COLOR_ID, Color::RED.into());

    let ship_size = Size(30.);
    let ship_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Triangle2d::default()).into(),
        material: NORMAL_SHIP_COLOR_ID,
        transform: Transform::default().with_scale(Vec3::splat(*ship_size * 2.)),
        ..Default::default()
    };
    let thrust_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(0.3, 0.5)).into(),
        material: NORMAL_SHIP_COLOR_ID,
        transform: Transform::default().with_translation(Vec3::new(0., -0.3, -0.1)),
        ..Default::default()
    };
    let rotation = Rotation(Quat::default());
    let thrust = Thrust(false);
    let velocity = Velocity(Vec3::zeroed());
    let ship_thruster = commands.spawn((thrust_mesh,)).id();
    let firing = Firing(false);
    let firing_timer = FiringTimer(Timer::new(
        Duration::from_millis(250),
        bevy::time::TimerMode::Once,
    ));

    let mut ship = commands.spawn((
        Position(Vec3::zeroed()),
        ship_mesh,
        RotateSpeed(0.),
        rotation,
        thrust,
        velocity,
        Ship,
        firing,
        firing_timer,
        Collidable,
        ship_size,
        AudioBundle {
            source: asset_server.load("thrust.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                paused: true,
                ..Default::default()
            },
        },
    ));

    ship.add_child(ship_thruster);
}

pub fn change_thruster_colors(
    mut query: Query<(&Thrust, &AudioSink, &mut Children)>,
    mut commands: Commands,
) {
    for (thrust, audio_sink, children) in &mut query {
        let thrusters = children
            .first()
            .expect("couldn't find the first child, which should be thrusters");

        if thrust.0 {
            commands.entity(*thrusters).insert(THRUSTING_SHIP_COLOR_ID);
            audio_sink.play();
        } else {
            commands.entity(*thrusters).insert(NORMAL_SHIP_COLOR_ID);
            audio_sink.pause();
        }
    }
}

pub fn rotate_ship(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &RotateSpeed, &mut Rotation)>,
) {
    for (mut transform, rotate_speed, mut rotation) in &mut query {
        transform.rotate_z(rotate_speed.0 * time.delta_seconds());
        rotation.0 = transform.rotation;
    }
}

pub fn input_rotate_ship(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut RotateSpeed>,
) {
    for mut rotate_speed in &mut query {
        rotate_speed.0 = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            2.0
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            -2.0
        } else {
            0.0
        };

        rotate_speed.0 = rotate_speed.0.clamp(-5., 5.);
    }
}

pub fn input_thrust_ship(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Thrust>) {
    for mut thrust in &mut query {
        thrust.0 = keyboard_input.pressed(KeyCode::ArrowUp);
    }
}

pub fn input_firing(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Firing>) {
    let Ok(mut firing) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Space) {
        firing.0 = true;
    } else {
        firing.0 = false;
    }
}

pub fn apply_thrust(mut query: Query<(&Thrust, &mut Velocity, &Transform)>) {
    for (thrust, mut velocity, transform) in &mut query {
        if thrust.0 {
            let acceleration = 1.;

            let direction = transform.rotation * Vec3::Y;
            let direction = direction.normalize();

            velocity.0 += acceleration * direction;
        }
    }
}

pub fn handle_ship_collisions(
    asteroid_query: Query<(&Position, &Size, Entity), With<Asteroid>>,
    ship_query: Query<(&Position, &Size, Entity), With<Ship>>,
    mut bevy_commands: Commands,
    mut explosion_event: EventWriter<ExplosionEvent>,
) {
    for (ship_position, ship_size, ship_entity) in ship_query.iter() {
        for (asteroid_position, asteroid_size, _asteroid_entity) in asteroid_query.iter() {
            if ship_position.distance(**asteroid_position) > **ship_size + **asteroid_size {
                continue;
            }

            bevy_commands.entity(ship_entity).despawn_recursive();
            explosion_event.send(ExplosionEvent(ship_position.clone()));
            break;
        }
    }
}

pub fn teleport_ship(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    world_size: Res<WorldSize>,
    mut ship_query: Query<&mut Position, With<Ship>>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::Enter) {
        if let Some(mut ship_position) = ship_query.get_single_mut().ok() {
            ship_position.set_random(&*world_size);
        }
    }
}
pub fn handle_ship_bullet_collisions(
    ship_query: Query<(&Position, &Size, Entity), With<Ship>>,
    bullet_query: Query<(&Position, &Size, Entity), With<UfoBullet>>,
    mut commands: Commands,
    mut explosion_event: EventWriter<ExplosionEvent>,
) {
    for (bullet_position, bullet_size, bullet_entity) in bullet_query.iter() {
        for (ship_position, ship_size, ship) in ship_query.iter() {
            if bullet_position.distance(**ship_position) > **bullet_size + **ship_size {
                continue;
            }
            commands.entity(ship).despawn_recursive();
            commands.entity(bullet_entity).despawn();

            explosion_event.send(ExplosionEvent(ship_position.clone()));

            break; // Each bullet can only hit one asteroid
        }
    }
}
