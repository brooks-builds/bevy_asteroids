use crate::{
    components::{Asteroid, Bullet, Collidable, Position, Ship, Size, Velocity},
    events::{ExplosionEvent, ScoreEvent},
    resources::{AsteroidCount, WorldSize},
    states::GameState,
};
use bevy::{prelude::*, render::color};
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    shapes::{self, Polygon},
};
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::ops::Deref;

pub fn spawn_asteroids(
    mut commands: Commands,
    world_size: Res<WorldSize>,
    ship_query: Query<&Position, With<Ship>>,
    desired_asteroids: Res<AsteroidCount>,
) {
    let ship_position = ship_query.iter().next();

    let world_size: Vec2 = world_size.deref().into();
    let mut rng = thread_rng();
    let mut created_asteroids = 0;
    let size = Size::from_scale(2.);

    let half_width = world_size.x / 2.;
    let half_height = world_size.y / 2.;

    loop {
        let position = Position(Vec3 {
            x: rng.gen_range(-half_width..half_width),
            y: rng.gen_range(-half_height..half_height),
            z: 0.,
        });

        if let Some(ship_position) = ship_position {
            let distance_to_ship = position.distance(**ship_position);

            if distance_to_ship < *size * 2. {
                continue;
            }
        };

        spawn_asteroid(&mut commands, 2., &mut rng, position);
        created_asteroids += 1;

        if created_asteroids >= **desired_asteroids {
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
    asteroid_query: Query<(&Position, &Size, Entity), With<Asteroid>>,
    bullet_query: Query<(&Position, &Size, Entity), With<Bullet>>,
    mut commands: Commands,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut score_event: EventWriter<ScoreEvent>,
) {
    for (bullet_position, bullet_size, bullet_entity) in bullet_query.iter() {
        for (asteroid_position, asteroid_size, asteroid_entity) in asteroid_query.iter() {
            if bullet_position.distance(**asteroid_position) > **bullet_size + **asteroid_size {
                continue;
            }
            commands.entity(asteroid_entity).despawn();
            commands.entity(bullet_entity).despawn();

            explosion_event.send(ExplosionEvent(asteroid_position.clone()));
            score_event.send(ScoreEvent(1));
            // create 2 asteroids

            let mut rng = thread_rng();

            let scale = asteroid_size.to_scale() / 2.;
            if scale > 0.1 {
                spawn_asteroid(&mut commands, scale, &mut rng, asteroid_position.clone());
                spawn_asteroid(&mut commands, scale, &mut rng, asteroid_position.clone());
            }

            break; // Each bullet can only hit one asteroid
        }
    }
}

pub fn end_level(
    asteroids_query: Query<&Asteroid>,
    mut asteroid_count: ResMut<AsteroidCount>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !asteroids_query.is_empty() {
        return;
    }

    asteroid_count.0 += 1;

    if let GameState::Playing = current_state.get() {
        next_state.set(GameState::GetReady);
    }
}
