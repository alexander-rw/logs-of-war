use bevy::prelude::*;

use crate::resources::{
    game_state_event::GameStateEvent,
    game_state::GameState,
    map_selection::MapSelection,
};

pub fn game_flow_plugin(app: &mut App) {
    app.add_message::<GameStateEvent>()
        .add_systems(Update, handle_game_flow_events);
}

fn handle_game_flow_events(
    mut events: MessageReader<GameStateEvent>,
    mut game_state: ResMut<NextState<GameState>>,
    mut map_selection: ResMut<MapSelection>,
) {
    for event in events.read() {
        match event {
            GameStateEvent::SplashComplete => {
                game_state.set(GameState::Menu);
            }
            GameStateEvent::PlayRequested => {
                game_state.set(GameState::PreGame);
            }
            GameStateEvent::MapSelected(map) => {
                *map_selection = *map;
                info!("Building map: {:?}", map);
                game_state.set(GameState::Game);
            }
            GameStateEvent::GameComplete => {
                game_state.set(GameState::Menu);
            }
        }
    }
}
