mod components;
pub mod events;
mod resources;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use events::{Collision, ExplosionEvent};
use resources::WorldSize;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, Game))
        .run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_event::<Collision>();
        app.add_event::<ExplosionEvent>();
        app.insert_resource(WorldSize(1920., 1080.));
        app.add_systems(
            Startup,
            (
                systems::ship_systems::add_player,
                systems::camera_systems::add_camera,
            ),
        );
        app.add_systems(PostStartup, (systems::asteroid_systems::spawn_asteroids,));
        app.add_systems(
            Update,
            (
                systems::ship_systems::change_thruster_colors,
                (
                    systems::camera_systems::update_world_size,
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
                    systems::shared_systems::detect_collisions,
                    // systems::debug_systems::visualize_size,
                    systems::ship_systems::handle_ship_collisions,
                    systems::asteroid_systems::handle_collisions,
                    systems::explosion::handle_explosion_event,
                    systems::explosion::remove_explosion,
                    systems::explosion::update_explosion,
                )
                    .chain(),
            ),
        );
    }
}
