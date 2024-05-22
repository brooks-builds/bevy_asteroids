use bevy::ecs::schedule::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Starting,
    GetReady,
    Playing,
    GameOver,
    Boss,
}
