use crate::{
    components::{Bullet, Collidable, FiringTimer, Position, ShipBullet, Size, Velocity, UFO},
    events::{ExplosionEvent, ScoreEvent},
    resources::{UfoTimer, WorldSize},
};
use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Stroke, entity::ShapeBundle, geometry::GeometryBuilder, shapes};
use rand::{thread_rng, Rng};

pub fn set_ufo_spawn_timer(mut ufo_spawn_timer: ResMut<UfoTimer>) {
    ufo_spawn_timer.reset()
}

pub fn ufo_spawn_timer_update(mut ufo_spawn_timer: ResMut<UfoTimer>, timer: Res<Time>) {
    ufo_spawn_timer.tick(timer.delta());
}

pub fn spawn_ufo(
    ufo_spawn_timer: Res<UfoTimer>,
    mut commands: Commands,
    world_size: Res<WorldSize>,
) {
    if !ufo_spawn_timer.just_finished() {
        return;
    }

    let size = Size(50.);
    let shape = create_ufo_shape(*size);

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Stroke::new(Color::ANTIQUE_WHITE, 2.),
        UFO,
        Position::new_random_edge(&*world_size),
        size,
        Velocity(Vec3::ZERO),
        Collidable,
        FiringTimer(Timer::from_seconds(0.5, TimerMode::Once)),
    ));
}

fn create_ufo_shape(size: f32) -> bevy_prototype_lyon::shapes::Polygon {
    let size = Vec2::new(size, size * 0.85);
    let half_size = size * 0.5;
    let points = vec![
        Vec2::new(-size.x, 0.),
        Vec2::new(-half_size.x, half_size.y),
        half_size,
        Vec2::new(size.x, 0.),
        Vec2::new(half_size.x, -half_size.y),
        -half_size,
        Vec2::new(-size.x, 0.),
        Vec2::new(size.x, 0.),
    ];

    shapes::Polygon {
        points,
        closed: false,
    }
}

pub fn update_velocity(mut ufo_query: Query<&mut Velocity, With<UFO>>) {
    let mut rng = thread_rng();
    let force = Vec3::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.);

    for mut velocity in &mut ufo_query {
        **velocity += force;
    }
}

pub fn handle_ufo_bullet_collisions(
    ufo_query: Query<(&Position, &Size, Entity), With<UFO>>,
    bullet_query: Query<(&Position, &Size, Entity, &ShipBullet), With<Bullet>>,
    mut commands: Commands,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    for (bullet_position, bullet_size, bullet_entity, _ship_marker) in bullet_query.iter() {
        for (ufo_position, ufo_size, ufo) in ufo_query.iter() {
            if bullet_position.distance(**ufo_position) > **bullet_size + **ufo_size {
                continue;
            }
            commands.entity(ufo).despawn();
            commands.entity(bullet_entity).despawn();

            explosion_event.send(ExplosionEvent(ufo_position.clone()));
            score_event.send(ScoreEvent(10));

            break; // Each bullet can only hit one asteroid
        }
    }
}

pub fn update(mut ufo_query: Query<&mut FiringTimer, With<UFO>>, timer: Res<Time>) {
    for mut firing_timer in &mut ufo_query {
        firing_timer.tick(timer.delta());
    }
}
