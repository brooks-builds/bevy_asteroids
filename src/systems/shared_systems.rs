use std::ops::Deref;

use crate::{
    components::*,
    events::Collision,
    resources::{AsteroidCount, WorldSize},
    states::GameState,
};
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

pub fn wraparound_entities(mut query: Query<&mut Position>, world_size: Res<WorldSize>) {
    let mut world_size: Vec2 = world_size.deref().into();
    world_size /= 2.;

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

pub fn transition_states(
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::Space) {
        match current_game_state.get() {
            GameState::Starting => next_game_state.set(GameState::GetReady),
            GameState::GetReady => unreachable!(),
            GameState::Playing => unreachable!(),
            GameState::GameOver => next_game_state.set(GameState::Starting),
            GameState::Boss => todo!(),
        };
    }
}

pub fn reset_game(
    mut asteroid_count: ResMut<AsteroidCount>,
    mut asteroids_query: Query<Entity, With<Asteroid>>,
    mut commands: Commands,
) {
    asteroid_count.0 = 1;

    for asteroid in &mut asteroids_query {
        commands.entity(asteroid).despawn();
    }
}

pub fn reset_ui(query: Query<Entity, With<UI>>, mut commands: Commands) {
    for ui in &query {
        commands.entity(ui).despawn();
    }
}
