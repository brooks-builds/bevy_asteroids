use crate::components::{Collidable, Position, Size};
use bevy::gizmos::gizmos::Gizmos;
use bevy::prelude::*;

#[allow(unused)]
pub fn visualize_size(mut gizmos: Gizmos, query: Query<(&Position, &Size), With<Collidable>>) {
    for (position, size) in &query {
        gizmos.circle_2d(
            position.into(),
            size.0,
            Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.3,
            },
        );
    }
}
