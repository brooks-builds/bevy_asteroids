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

pub fn update_explosion(mut explosion_query: Query<(&mut Transform, &Position, &mut Stroke), With<Explosion>>) {
    for (mut transform, position, mut stroke) in &mut explosion_query {
        transform.scale *= 1.1;
        transform.translation = **position;
        transform.
    }
}

pub fn remove_explosion() {}
