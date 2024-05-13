use bevy::{prelude::*, render::camera::ScalingMode};

use crate::components::MainCamera;

pub fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::WindowSize(1.0);

    commands.spawn((camera, MainCamera));
}
