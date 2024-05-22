use crate::components::Position;
use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ExplosionEvent(pub Position);

#[derive(Event)]
pub struct ScoreEvent(pub u16);
