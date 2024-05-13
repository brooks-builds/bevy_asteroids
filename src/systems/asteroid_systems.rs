use bevy::{prelude::*, render::color};
use bevy_prototype_lyon::{
    draw::Stroke,
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    shapes::{self, Polygon},
};
use rand::{thread_rng, Rng};

use crate::components::{Asteroid, MainCamera, Position, Velocity};

pub fn spawn_asteroids(
    mut commands: Commands,
    asteroids: Query<&Asteroid>,
    camera_query: Query<&Camera, With<MainCamera>>,
) {
    if asteroids.iter().count() > 0 {
        return;
    }

    let camera = camera_query.single();
    let world_size = camera
        .logical_viewport_rect()
        .expect("trying to get viewport size in wraparound entities system")
        .half_size();
    let asteroids_count = 5;
    let mut rng = thread_rng();
    let asteroid_speed = 15.;

    for _ in 0..asteroids_count {
        let shape = create_asteroid_shape(2.);
        let position = Position(Vec3 {
            x: rng.gen_range(-world_size.x..world_size.x),
            y: rng.gen_range(-world_size.y..world_size.y),
            z: 0.,
        });
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
        ));
    }
}

fn create_asteroid_shape(scale: f32) -> Polygon {
    let mut rng = thread_rng();
    let mut points = vec![];
    let point_count = 25.;

    for i in 0..point_count as u8 {
        let angle = i as f32 * std::f32::consts::TAU / point_count;
        let mut point = Vec2::from_angle(angle);

        point *= rng.gen_range(50.0..75.0);
        point *= scale;

        points.push(point);
    }

    shapes::Polygon {
        points,
        closed: true,
    }
}
