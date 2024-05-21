use bevy::{
    ecs::system::Resource,
    math::Vec2,
    prelude::{Deref, DerefMut},
};

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
