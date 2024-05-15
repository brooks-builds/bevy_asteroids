use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct Collision(pub Entity, pub Entity);
