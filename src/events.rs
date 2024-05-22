use crate::components::Position;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ExplosionEvent(pub Position);
