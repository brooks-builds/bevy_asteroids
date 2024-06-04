mod components;
pub mod events;
pub mod resources;
mod states;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bot::Bot;
use events::{ExplosionEvent, ScoreEvent};
use resources::{AsteroidCount, BeforeBossState, Countdown, HighScore, Score, UfoTimer, WorldSize};
use states::GameState;

const GET_READY_TIME: f32 = 4.;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, Game::new()))
        .run();
}

struct Game {
    bot: Bot,
}

impl Game {
    pub fn new() -> Self {
        let bot = Bot::new(8);

        Self { bot }
    }
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEvent>();
        app.add_event::<ScoreEvent>();

        app.insert_resource(WorldSize(1920., 1080.));
        app.insert_resource(AsteroidCount(10));
        app.insert_resource(BeforeBossState(GameState::Starting));
        app.insert_resource(Countdown(Timer::from_seconds(
            GET_READY_TIME,
            TimerMode::Once,
        )));
        app.insert_resource(Score(0));
        app.insert_resource(HighScore(0));
        app.insert_resource(UfoTimer(Timer::from_seconds(15., TimerMode::Once)));

        app.insert_state(GameState::Starting);

        app.add_systems(
            Startup,
            (
                systems::camera_systems::add_camera,
                systems::camera_systems::add_camera_border,
                systems::ui::display_score.after(systems::shared_systems::load_high_score),
                systems::shared_systems::load_high_score,
                set_speed,
            ),
        );

        app.add_systems(
            OnEnter(GameState::Starting),
            (
                systems::asteroid_systems::spawn_asteroids,
                systems::ui::title_screen,
            ),
        );

        app.add_systems(
            OnExit(GameState::Starting),
            (
                systems::shared_systems::reset_game,
                systems::shared_systems::reset_ui,
            ),
        );

        app.add_systems(
            OnExit(GameState::Playing),
            (systems::shared_systems::save_high_score,),
        );

        app.add_systems(
            OnEnter(GameState::GetReady),
            (
                systems::ship_systems::add_player,
                systems::asteroid_systems::spawn_asteroids.after(systems::ship_systems::add_player),
                systems::shared_systems::update_positions
                    .after(systems::asteroid_systems::spawn_asteroids),
                systems::shared_systems::reset_countdown,
                systems::ui::get_ready_screen,
                systems::bullet_systems::delete_all_bullets,
            ),
        );

        app.add_systems(
            OnExit(GameState::GetReady),
            (
                systems::shared_systems::reset_ui,
                systems::ufo_systems::set_ufo_spawn_timer,
            ),
        );

        app.add_systems(
            Update,
            (
                (
                    systems::shared_systems::apply_velocity,
                    systems::shared_systems::update_positions,
                    systems::shared_systems::wraparound_entities,
                    systems::shared_systems::transition_states,
                )
                    .run_if(in_state(GameState::Starting)),
                (
                    systems::shared_systems::tick_countdown,
                    systems::ui::update_get_ready_screen,
                    systems::shared_systems::transition_from_get_ready_to_playing,
                    systems::explosion::update_explosion,
                    systems::explosion::remove_explosion,
                    systems::ui::update_score_ui,
                )
                    .run_if(in_state(GameState::GetReady)),
                (
                    systems::ship_systems::change_thruster_colors,
                    (
                        systems::shared_systems::wraparound_entities,
                        systems::ship_systems::input_rotate_ship,
                        systems::ship_systems::rotate_ship,
                        systems::ship_systems::input_thrust_ship,
                        systems::ship_systems::apply_thrust,
                        systems::shared_systems::apply_velocity,
                        systems::ship_systems::input_firing,
                        systems::bullet_systems::ship_fire_bullet,
                        systems::bullet_systems::delete_expired_bullets,
                        // systems::debug_systems::visualize_size,
                        systems::ship_systems::handle_ship_collisions,
                        systems::asteroid_systems::handle_collisions,
                        systems::explosion::handle_explosion_event,
                        systems::explosion::remove_explosion,
                        systems::explosion::update_explosion,
                        systems::shared_systems::update_positions,
                    )
                        // this chain tells the above systems that they need to run in order
                        .chain(),
                    systems::shared_systems::transition_from_playing_to_game_over,
                    systems::shared_systems::update_scores,
                    systems::ui::update_score_ui,
                    systems::asteroid_systems::end_level,
                    systems::ship_systems::teleport_ship,
                    systems::ufo_systems::ufo_spawn_timer_update,
                    systems::ufo_systems::spawn_ufo,
                    systems::ufo_systems::update_velocity,
                    systems::ufo_systems::handle_ufo_bullet_collisions,
                    systems::ufo_systems::update,
                    systems::bullet_systems::ufo_fire_bullet,
                    systems::ship_systems::handle_ship_bullet_collisions
                        .after(systems::ship_systems::change_thruster_colors),
                )
                    .run_if(in_state(GameState::Playing)),
                (
                    systems::shared_systems::transition_states,
                    systems::explosion::update_explosion,
                    systems::explosion::remove_explosion,
                )
                    .run_if(in_state(GameState::GameOver)),
                (systems::shared_systems::to_from_boss,),
            ),
        );

        app.add_systems(
            OnEnter(GameState::GameOver),
            (systems::ui::game_over_screen,),
        );

        app.add_systems(
            OnExit(GameState::GameOver),
            (
                systems::shared_systems::reset_ui,
                systems::shared_systems::reset_game,
            ),
        );
    }
}

fn set_speed(mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(1.0);
}
