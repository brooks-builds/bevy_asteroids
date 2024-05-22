use bevy::{
    ecs::system::{Commands, Query, Res},
    text::{Text, TextSection, TextStyle},
    ui::{
        node_bundles::{self, TextBundle},
        Style, UiRect,
    },
};

use crate::{components::UI, resources::Countdown, GET_READY_TIME};

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

pub fn get_ready_screen(mut commands: Commands) {
    let title = "Get Ready";
    let subtitle = format!("\n{}", 3);

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                title,
                TextStyle {
                    font_size: 100.,
                    ..Default::default()
                },
            ),
            TextSection::new(
                subtitle,
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
            margin: UiRect::top(bevy::ui::Val::Vh(25.)),
            ..Default::default()
        }),
        UI,
    ));
}

pub fn update_get_ready_screen(mut query: Query<&mut Text>, countdown: Res<Countdown>) {
    let time_remaining = (GET_READY_TIME - countdown.elapsed_secs()) as u8;
    let mut text = query.single_mut();

    text.sections[1].value = format!("\n{time_remaining}");
}

pub fn game_over_screen(mut commands: Commands) {
    let title = "Game Over";
    let subtitle = format!("\nScore: {}\nPress space to try again", 3);

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                title,
                TextStyle {
                    font_size: 100.,
                    ..Default::default()
                },
            ),
            TextSection::new(
                subtitle,
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
            margin: UiRect::top(bevy::ui::Val::Vh(25.)),
            ..Default::default()
        }),
        UI,
    ));
}

pub fn boss_screen() {}
