use bevy::prelude::*;

use crate::resources::map_selection::MapSelection;

#[derive(Message)]
pub enum GameStateEvent {
    SplashComplete,
    PlayRequested,
    MapSelected(MapSelection),
    GameComplete,
}
