use bevy::{
    ecs::system::Resource,
    math::Vec2,
    prelude::{Deref, DerefMut},
    time::Timer,
};

use crate::states::GameState;

#[derive(Resource, Debug)]
pub struct WorldSize(pub f32, pub f32);

impl From<&WorldSize> for Vec2 {
    fn from(value: &WorldSize) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct AsteroidCount(pub u8);

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Countdown(pub Timer);

#[derive(Resource, Debug, Deref, DerefMut, Copy, Clone)]
pub struct BeforeBossState(pub GameState);

#[derive(Resource, Debug, Deref, DerefMut, Clone, Copy)]
pub struct Score(pub u16);

impl From<Score> for String {
    fn from(value: Score) -> Self {
        value.to_string()
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Clone, Copy)]
pub struct HighScore(pub u16);

impl From<HighScore> for String {
    fn from(value: HighScore) -> Self {
        value.to_string()
    }
}
