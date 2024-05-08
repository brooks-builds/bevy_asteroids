use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

#[derive(Component)]
pub struct Position(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct RotateSpeed(pub f32);

#[derive(Component)]
pub struct Rotation(pub Quat);

#[derive(Component)]
pub struct Thrust(pub bool);

#[derive(Component)]
pub struct MainCamera;
