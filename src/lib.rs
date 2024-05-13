mod components;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, Game))
        .run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                systems::ship_systems::add_player,
                systems::camera_systems::add_camera,
            ),
        );
        app.add_systems(
            Update,
            (
                systems::ship_systems::change_thruster_colors,
                (
                    systems::ship_systems::input_rotate_ship,
                    systems::ship_systems::rotate_ship,
                    systems::ship_systems::input_thrust_ship,
                    systems::ship_systems::apply_thrust,
                    systems::shared_systems::apply_velocity,
                    systems::shared_systems::wraparound_entities,
                    systems::ship_systems::input_firing,
                    systems::bullet_systems::fire_bullet,
                    systems::shared_systems::update_positions,
                    systems::bullet_systems::delete_expired_bullets,
                    systems::asteroid_systems::spawn_asteroids,
                )
                    .chain(),
            ),
        );
    }
}
