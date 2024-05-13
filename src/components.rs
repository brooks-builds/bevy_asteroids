use bevy::{
    ecs::component::Component,
    math::{bounding::BoundingCircle, Quat, Vec3},
    time::Timer,
};

#[derive(Component, Clone, Debug)]
pub struct Position(pub Vec3);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct RotateSpeed(pub f32);

#[derive(Component, Clone)]
pub struct Rotation(pub Quat);

#[derive(Component)]
pub struct Thrust(pub bool);

#[derive(Component)]
pub struct Firing(pub bool);

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct FiringTimer(pub Timer);

#[derive(Component)]
pub struct BulletTimer(pub Timer);

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct CollisionDetection(pub BoundingCircle);
