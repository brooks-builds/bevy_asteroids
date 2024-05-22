use std::ops::Deref;

use crate::{
    components::*,
    events::ScoreEvent,
    resources::{AsteroidCount, BeforeBossState, Countdown, HighScore, Score, WorldSize},
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
            GameState::GameOver => next_game_state.set(GameState::GetReady),
            GameState::Boss => todo!(),
        };
    }
}

pub fn reset_game(
    mut asteroid_count: ResMut<AsteroidCount>,
    mut entities_to_despawn: Query<Entity, Or<(With<Bullet>, With<Asteroid>)>>,
    mut commands: Commands,
) {
    asteroid_count.0 = 1;

    for entity in &mut entities_to_despawn {
        commands.entity(entity).despawn();
    }
}

pub fn reset_ui(query: Query<Entity, With<UI>>, mut commands: Commands) {
    for ui in &query {
        commands.entity(ui).despawn();
    }
}

pub fn reset_countdown(mut countdown: ResMut<Countdown>) {
    countdown.reset();
}

pub fn tick_countdown(mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.tick(time.delta());
}

pub fn transition_from_get_ready_to_playing(
    countdown: Res<Countdown>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_game_state: Res<State<GameState>>,
) {
    if !countdown.finished() {
        return;
    }

    match current_game_state.get() {
        GameState::GetReady => next_game_state.set(GameState::Playing),
        _ => unreachable!(),
    };
}

pub fn transition_from_playing_to_game_over(
    ship_query: Query<Entity, With<Ship>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if !ship_query.is_empty() {
        return;
    }

    next_game_state.set(GameState::GameOver);
}

pub fn to_from_boss(
    current_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    mut before_boss_state: ResMut<BeforeBossState>,
) {
    if keyboard_input.clear_just_pressed(KeyCode::KeyB) {
        match current_game_state.get() {
            GameState::Boss => next_game_state.set(**before_boss_state),
            GameState::GetReady => (),
            _ => {
                before_boss_state.0 = *current_game_state.get();
                next_game_state.set(GameState::Boss);
                open::that("https://updatefaker.com/").ok();
            }
        }
    }
}

pub fn update_scores(
    mut score_events: EventReader<ScoreEvent>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
) {
    for &ScoreEvent(value) in score_events.read() {
        **score += value;

        **high_score = high_score.max(**score);
    }
}
