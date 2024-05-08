use std::time::Duration;

use crate::components::{
    Bullet, Firing, MainCamera, Position, RotateSpeed, Rotation, Ship, Thrust, Velocity,
};
use bevy::{
    asset::{Assets, Handle},
    core::Zeroable,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::{With, Without},
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children},
    input::{keyboard::KeyCode, ButtonInput},
    math::{
        primitives::{Circle, Rectangle, Triangle2d},
        Quat, Vec3,
    },
    render::{
        camera::{Camera, ScalingMode},
        color::Color,
        mesh::Mesh,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::{Time, Timer},
    transform::components::Transform,
};

const NORMAL_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(389743489572398);
const THRUSTING_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(38475109234891725);

pub fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);

    commands.spawn((camera, MainCamera));
}

pub fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    materials.insert(NORMAL_SHIP_COLOR_ID, Color::ANTIQUE_WHITE.into());
    materials.insert(THRUSTING_SHIP_COLOR_ID, Color::RED.into());

    let ship_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Triangle2d::default()).into(),
        material: NORMAL_SHIP_COLOR_ID,
        transform: Transform::default().with_scale(Vec3::splat(50.)),
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
    let ship_thruster = commands.spawn(thrust_mesh).id();
    let firing = Firing(false);

    let mut ship = commands.spawn((
        Position(Vec3::zeroed()),
        ship_mesh,
        RotateSpeed(0.),
        rotation,
        thrust,
        velocity,
        Ship,
        firing,
    ));

    ship.add_child(ship_thruster);
}

pub fn change_thruster_colors(mut query: Query<(&Thrust, &Children)>, mut commands: Commands) {
    for (thrust, children) in &mut query {
        let thrusters = children
            .first()
            .expect("couldn't find the first child, which should be thrusters");

        if thrust.0 {
            commands.entity(*thrusters).insert(THRUSTING_SHIP_COLOR_ID);
        } else {
            commands.entity(*thrusters).insert(NORMAL_SHIP_COLOR_ID);
        }
    }
}

pub fn update_positions(mut query: Query<(&mut Transform, &Position), Without<Ship>>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
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
            1.0
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            -1.0
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
    let mut firing = query.single_mut();

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

pub fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

pub fn wraparound_entities(
    mut query: Query<&mut Position>,
    camera_query: Query<&Camera, With<MainCamera>>,
) {
    let camera = camera_query.single();
    let world_size = camera
        .logical_viewport_rect()
        .expect("trying to get viewport size in wraparound entities system")
        .half_size();

    for mut position in &mut query {
        if position.0.x > world_size.x {
            position.0.x = -world_size.x;
        } else if position.0.x < -world_size.x {
            position.0.x = world_size.x;
        }

        if position.0.y > world_size.y {
            position.0.y = -world_size.y;
        } else if position.0.y < -world_size.y {
            position.0.y = world_size.y;
        }
    }
}

pub fn fire_bullet(
    mut commands: Commands,
    mut firing_query: Query<&mut Firing>,
    ship_query: Query<(&Position, &Rotation), With<Ship>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut firing = firing_query.single_mut();
    let (ship_position, ship_rotation) = ship_query.single();

    if firing.0 {
        firing.0 = false;
    } else {
        return;
    }

    let bullet_position = ship_position.clone();
    let bullet_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(Color::WHITE),
        transform: Transform::default()
            .with_scale(Vec3::splat(2.))
            .with_rotation(ship_rotation.0.clone()),
        ..Default::default()
    };
    let direction = (bullet_mesh.transform.rotation * Vec3::Y).normalize() * 500.;
    let bullet_velocity = Velocity(direction);

    commands.spawn((Bullet, bullet_position, bullet_velocity, bullet_mesh));
}
