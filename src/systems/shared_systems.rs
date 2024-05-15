use crate::{components::*, events::Collision};
use bevy::prelude::*;

pub fn update_positions(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}

pub fn apply_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

pub fn wraparound_entities(
    mut query: Query<&mut Position>,
    camera_query: Query<&Camera, With<MainCamera>>,
) {
    let camera = camera_query.single();
    let world_size = camera
        .logical_viewport_rect()
        .expect("trying to get viewport size in wraparound entities system")
        .half_size();

    for mut position in &mut query {
        if position.0.x > world_size.x {
            position.0.x = -world_size.x;
        } else if position.0.x < -world_size.x {
            position.0.x = world_size.x;
        }

        if position.0.y > world_size.y {
            position.0.y = -world_size.y;
        } else if position.0.y < -world_size.y {
            position.0.y = world_size.y;
        }
    }
}

pub fn detect_collisions(
    query: Query<(&Position, &Size, Entity), With<Collidable>>,
    mut collision_event: EventWriter<Collision>,
) {
    for (index, (position, size, entity)) in query.iter().enumerate() {
        for (other_position, other_size, other_entity) in query.iter().skip(index + 1) {
            let distance = position.distance(**other_position) - (**size) - (**other_size);

            if distance <= 0. {
                collision_event.send(Collision(entity, other_entity));
            }
        }
    }
}
