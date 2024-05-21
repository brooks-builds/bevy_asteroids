use crate::{
    components::{Explosion, Position},
    events::ExplosionEvent,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{draw::Stroke, entity::ShapeBundle, geometry::GeometryBuilder, shapes};

pub fn handle_explosion_event(
    mut explosion_event: EventReader<ExplosionEvent>,
    mut commands: Commands,
) {
    for explosion in explosion_event.read() {
        let position = explosion.0.clone();
        let shape = shapes::Circle {
            radius: 10.,
            center: Vec2::ZERO,
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..Default::default()
            },
            Stroke::new(Color::ANTIQUE_WHITE, 5.),
            Explosion,
            position,
        ));
    }
}

pub fn update_explosion(
    mut explosion_query: Query<(&mut Transform, &Position, &mut Stroke), With<Explosion>>,
) {
    for (mut transform, position, mut stroke) in &mut explosion_query {
        transform.scale *= 1.05;
        transform.translation = **position;

        let line_width = (stroke.options.line_width - 0.15).clamp(0., 5.);
        stroke.options.line_width = line_width;

        let alpha = (stroke.color.a() - 0.03).clamp(0., 100.);
        stroke.color = stroke.color.with_a(alpha);
    }
}

pub fn remove_explosion() {}
