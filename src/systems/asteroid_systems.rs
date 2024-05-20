use std::ops::Deref;

use crate::{
    components::{Asteroid, Bullet, Collidable, MainCamera, Position, Ship, Size, Velocity},
    events::Collision,
    resources::WorldSize,
};
use bevy::{prelude::*, render::color};
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    shapes::{self, Polygon},
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub fn spawn_asteroids(
    mut commands: Commands,
    world_size: Res<WorldSize>,
    ship_query: Query<(&Position, &Size), With<Ship>>,
) {
    let (ship_position, ship_size) = ship_query.single();

    let world_size: Vec2 = world_size.deref().into();
    let desired_asteroids = 1;
    let mut rng = thread_rng();
    let mut created_asteroids = 0;
    let size = Size::from_scale(2.);

    loop {
        let position = Position(Vec3 {
            x: rng.gen_range(-world_size.x..world_size.x),
            y: rng.gen_range(-world_size.y..world_size.y),
            z: 0.,
        });

        let distance_to_ship = position.distance(**ship_position);

        if distance_to_ship < *size * 2. {
            continue;
        }

        spawn_asteroid(&mut commands, 2., &mut rng, position);
        created_asteroids += 1;

        if created_asteroids >= desired_asteroids {
            break;
        }
    }
}

fn spawn_asteroid(commands: &mut Commands, scale: f32, rng: &mut ThreadRng, position: Position) {
    // scale = 2.0 == 15
    // scale = 1.0 == 30
    // scale = 0.5 == 60
    let asteroid_speed = rng.gen_range(29.0..31.0) / scale;
    let (shape, size) = create_asteroid_shape(scale);
    let velocity =
        Vec2::from_angle(rng.gen_range(0.0..std::f32::consts::TAU)).extend(0.) * asteroid_speed;

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Stroke::new(color::Color::ANTIQUE_WHITE, 2.),
        Asteroid,
        position,
        Velocity(velocity),
        Collidable,
        size,
    ));
}

fn create_asteroid_shape(scale: f32) -> (Polygon, Size) {
    let mut rng = thread_rng();
    let mut points = vec![];
    let point_count = 25.;
    let size = Size::from_scale(scale);

    for i in 0..point_count as u8 {
        let angle = i as f32 * std::f32::consts::TAU / point_count;
        let mut point = Vec2::from_angle(angle);

        point *= rng.gen_range(50.0..75.0);
        point *= scale;

        points.push(point);
    }

    (
        shapes::Polygon {
            points,
            closed: true,
        },
        size,
    )
}

pub fn handle_collisions(
    bullet_query: Query<&Bullet>,
    asteroid_query: Query<(&Asteroid, &Size, Entity, &Position)>,
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
) {
    for Collision(collided_a, collided_b) in collision_events.read() {
        let collided_entities = [collided_a, collided_b];
        let Some(&bullet) = collided_entities
            .into_iter()
            .find(|&&entity| bullet_query.get(entity).is_ok())
        else {
            continue;
        };
        let Some((_asteroid, asteroid_size, asteroid_entity, asteroid_position)) =
            collided_entities
                .into_iter()
                .find_map(|&entity| asteroid_query.get(entity).ok())
        else {
            continue;
        };
        commands.entity(asteroid_entity).despawn();
        commands.entity(bullet).despawn();
        // create 2 asteroids

        let mut rng = thread_rng();

        let scale = asteroid_size.to_scale() / 2.;
        if scale > 0.1 {
            spawn_asteroid(&mut commands, scale, &mut rng, asteroid_position.clone());
            spawn_asteroid(&mut commands, scale, &mut rng, asteroid_position.clone());
        }
    }
}

#[allow(unused)]
/// From @rasmusgo1 on Twitch
pub fn handle_ship_collisions(
    asteroid_query: Query<(&Position, &Size, Entity), With<Asteroid>>,
    ship_query: Query<(&Position, &Size, Entity), With<Ship>>,
    mut bevy_commands: Commands,
) {
    for (ship_position, ship_size, ship_entity) in ship_query.iter() {
        for (asteroid_position, asteroid_size, _asteroid_entity) in asteroid_query.iter() {
            let distance =
                ship_position.distance(**asteroid_position) - (**ship_size) - (**asteroid_size);
            if distance <= 0. {
                bevy_commands.entity(ship_entity).despawn_recursive();
            }
        }
    }
}
