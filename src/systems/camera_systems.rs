use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{components::MainCamera, resources::WorldSize};

pub fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);

    commands.spawn((camera, MainCamera));
}

pub fn update_world_size(mut camera_query: Query<&Camera>, mut world_size: ResMut<WorldSize>) {
    let camera = camera_query.single();
    let camera_size = camera.logical_viewport_rect().unwrap_or_default().size();

    world_size.0 = camera_size.x;
    world_size.1 = camera_size.y;
}
