use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Position(pub f32, pub f32);

#[derive(Component)]
pub struct RotateSpeed(pub f32);
