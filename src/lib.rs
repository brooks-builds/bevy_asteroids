mod components;
mod systems;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
    DefaultPlugins,
};
use systems::*;

pub fn run() {
    App::new().add_plugins((DefaultPlugins, Game)).run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_player, add_camera));
        app.add_systems(
            Update,
            (
                draw_ship,
                (
                    input_rotate_ship,
                    rotate_ship,
                    input_thrust_ship,
                    apply_thrust,
                    apply_velocity,
                    wraparound_entities,
                )
                    .chain(),
            ),
        );
    }
}
