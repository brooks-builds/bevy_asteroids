use crate::components::MainCamera;
use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::{draw::Stroke, entity::ShapeBundle, geometry::GeometryBuilder, shapes};

pub const WORLD_WIDTH: f32 = 1920.0;
pub const WORLD_HEIGHT: f32 = 1080.0;

pub fn add_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: WORLD_WIDTH,
        min_height: WORLD_HEIGHT,
    };

    commands.spawn((camera, MainCamera));
}

pub fn add_camera_border(mut commands: Commands) {
    let outline = shapes::Rectangle {
        extents: Vec2::new(WORLD_WIDTH, WORLD_HEIGHT),
        origin: shapes::RectangleOrigin::Center,
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&outline),
            ..Default::default()
        },
        Stroke::new(Color::ANTIQUE_WHITE, 5.),
    ));
}
