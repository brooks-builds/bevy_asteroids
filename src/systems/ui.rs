use bevy::{
    ecs::system::Commands,
    text::{TextSection, TextStyle},
    ui::{
        node_bundles::{self, TextBundle},
        Style,
    },
};

use crate::components::UI;

pub fn title_screen(mut commands: Commands) {
    let game_name = "Asteroids";
    let start = "\nPress space to start";

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                game_name,
                TextStyle {
                    font_size: 100.,
                    ..Default::default()
                },
            ),
            TextSection::new(
                start,
                TextStyle {
                    font_size: 25.,
                    ..Default::default()
                },
            ),
        ])
        .with_text_justify(bevy::text::JustifyText::Center)
        .with_style(Style {
            position_type: bevy::ui::PositionType::Absolute,
            justify_self: bevy::ui::JustifySelf::Center,
            align_self: bevy::ui::AlignSelf::Center,
            ..Default::default()
        }),
        UI,
    ));
}

pub fn playing_screen() {}

pub fn get_ready_screen() {}

pub fn game_over_screen() {}

pub fn boss_screen() {}
