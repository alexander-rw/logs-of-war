use bevy::prelude::*;

use crate::resources::map_selection::MapSelection;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Splash,
    Menu,
    PreGame,
    Game,
}

#[derive(Message)]
pub enum GameStateEvent {
    SplashComplete,
    PlayRequested,
    MapSelected(MapSelection),
    GameComplete,
}
