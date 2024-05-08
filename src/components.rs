use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
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
