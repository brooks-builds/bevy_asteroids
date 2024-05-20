use bevy::{
    ecs::component::Component,
    math::{bounding::BoundingCircle, Quat, Vec3},
    prelude::{Deref, DerefMut, Vec2},
    time::Timer,
};

#[derive(Component, Clone, Debug, Deref, DerefMut, Copy)]
pub struct Position(pub Vec3);

impl From<&Position> for Vec2 {
    fn from(value: &Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Component, Deref, DerefMut)]
pub struct RotateSpeed(pub f32);

#[derive(Component, Clone, Deref, DerefMut)]
pub struct Rotation(pub Quat);

#[derive(Component, Deref, DerefMut)]
pub struct Thrust(pub bool);

#[derive(Component, Deref, DerefMut)]
pub struct Firing(pub bool);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Ship;

#[derive(Component, Deref, DerefMut)]
pub struct FiringTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Deref, DerefMut)]
pub struct CollisionDetection(pub BoundingCircle);

#[derive(Component)]
pub struct Collidable;

#[derive(Component, Deref, DerefMut)]
pub struct Size(pub f32);

impl Size {
    pub fn from_scale(scale: f32) -> Self {
        Self(65. * scale)
    }

    pub fn to_scale(&self) -> f32 {
        **self / 65.
    }
}
