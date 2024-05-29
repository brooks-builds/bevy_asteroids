use std::time::Duration;

use crate::components::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn ship_fire_bullet(
    mut commands: Commands,
    mut firing_query: Query<&mut Firing>,
    mut ship_query: Query<(&Position, &Rotation, &mut FiringTimer, &Velocity), With<Ship>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    bullet_query: Query<&Bullet, With<ShipBullet>>,
    time: Res<Time>,
) {
    let Ok(mut firing) = firing_query.get_single_mut() else {
        return;
    };
    let (ship_position, ship_rotation, mut firing_timer, ship_velocity) = ship_query.single_mut();
    let bullets = bullet_query.iter().count();

    firing_timer.0.tick(time.delta());

    if bullets >= 3 {
        firing.0 = false;
        return;
    }

    if firing.0 {
        firing.0 = false;
    } else {
        return;
    }

    if firing_timer.0.finished() {
        firing_timer.0.reset();
    } else {
        return;
    }

    let bullet_size = Size(7.5);
    let bullet_position = ship_position.clone();
    let bullet_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(Color::WHITE),
        transform: Transform::default()
            .with_scale(Vec3::splat(*bullet_size))
            .with_rotation(ship_rotation.0.clone()),
        ..Default::default()
    };
    let mut direction = (bullet_mesh.transform.rotation * Vec3::Y).normalize() * 1000.;

    direction += ship_velocity.0;

    let bullet_velocity = Velocity(direction);
    let bullet_timer = BulletTimer(Timer::new(
        Duration::from_millis(1000),
        bevy::time::TimerMode::Once,
    ));

    commands.spawn((
        Bullet,
        bullet_position,
        bullet_velocity,
        bullet_mesh,
        bullet_timer,
        Collidable,
        bullet_size,
        ShipBullet,
    ));
}

pub fn delete_expired_bullets(
    mut query: Query<(&mut BulletTimer, Entity)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut bullet_timer, entity_id) in &mut query {
        bullet_timer.0.tick(time.delta());

        if bullet_timer.0.finished() {
            commands.entity(entity_id).despawn();
        }
    }
}

pub fn delete_all_bullets(bullet_query: Query<Entity, With<Bullet>>, mut commands: Commands) {
    for bullet in &bullet_query {
        commands.entity(bullet).despawn();
    }
}
pub fn ufo_fire_bullet(
    mut commands: Commands,
    mut ufo_query: Query<(&Position, &mut FiringTimer), With<UFO>>,
    ship_query: Query<&Position, With<Ship>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Some(ship_position) = ship_query.get_single().ok() else {
        return;
    };
    let Some((ufo_position, mut ufo_firing_timer)) = ufo_query.get_single_mut().ok() else {
        return;
    };

    if !ufo_firing_timer.finished() {
        return;
    }

    ufo_firing_timer.reset();

    let bullet_size = Size(7.5);
    let bullet_position = ufo_position.clone();
    let bullet_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        material: materials.add(Color::ALICE_BLUE),
        transform: Transform::default().with_scale(Vec3::splat(*bullet_size)),
        ..Default::default()
    };
    let direction = (**ship_position - **ufo_position).normalize() * 1000.;

    let bullet_velocity = Velocity(direction);
    let bullet_timer = BulletTimer(Timer::new(
        Duration::from_millis(1000),
        bevy::time::TimerMode::Once,
    ));

    commands.spawn((
        Bullet,
        bullet_position,
        bullet_velocity,
        bullet_mesh,
        bullet_timer,
        Collidable,
        bullet_size,
        UfoBullet,
    ));
}
