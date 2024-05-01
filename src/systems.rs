use bevy::{
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, Query, Res, ResMut},
    input::{keyboard::KeyCode, ButtonInput},
    math::{primitives::Triangle2d, Vec3},
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
    transform::components::Transform,
};

use crate::components::{Position, RotateSpeed};

pub fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn add_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let ship_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Triangle2d::default()).into(),
        material: materials.add(Color::ORANGE_RED),
        transform: Transform::default().with_scale(Vec3::splat(100.)),
        ..Default::default()
    };

    commands.spawn((Position(50.0, 50.0), ship_mesh, RotateSpeed(0.)));
}

pub fn draw_ship(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.0;
        transform.translation.y = position.1;
    }
}

pub fn rotate_ship(time: Res<Time>, mut query: Query<(&mut Transform, &RotateSpeed)>) {
    for (mut transform, rotate_speed) in &mut query {
        transform.rotate_z(rotate_speed.0 * time.delta_seconds());
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
