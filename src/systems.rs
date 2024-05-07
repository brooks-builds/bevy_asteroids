use crate::components::{Position, RotateSpeed, Rotation, Thrust, Velocity};
use bevy::{
    asset::{Assets, Handle},
    core::Zeroable,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Query, Res, ResMut},
    hierarchy::{BuildChildren, Children},
    input::{keyboard::KeyCode, ButtonInput},
    math::{
        primitives::{Rectangle, Triangle2d},
        Quat, Vec3,
    },
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
    transform::components::Transform,
};

const NORMAL_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(389743489572398);
const THRUSTING_SHIP_COLOR_ID: Handle<ColorMaterial> = Handle::weak_from_u128(38475109234891725);

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
        transform: Transform::default().with_scale(Vec3::splat(100.)),
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

    let mut ship = commands.spawn((
        Position(Vec3::zeroed()),
        ship_mesh,
        RotateSpeed(0.),
        rotation,
        thrust,
        velocity,
    ));

    ship.add_child(ship_thruster);
}

pub fn draw_ship(
    mut query: Query<(&mut Transform, &Position, &Thrust, &Children)>,
    mut commands: Commands,
) {
    for (mut transform, position, thrust, children) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;

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
