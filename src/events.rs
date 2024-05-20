use crate::components::Position;
use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct Collision(pub Entity, pub Entity);

#[derive(Event)]
pub struct ExplosionEvent(pub Position);
