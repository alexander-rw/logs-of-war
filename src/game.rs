use avian3d::prelude::*;
use bevy::prelude::*;

use crate::battle::MapBattlePlugin;
use crate::briefing::briefing_plugin;
use crate::camera::setup_camera;
use crate::character::controller::CharacterControllerPlugin;
use crate::maps::MapSelection;
use crate::menu::menu_plugin;
use crate::physics::PhysicsBasePlugin;
use crate::splash::splash_plugin;

/// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Briefing,
    Battle,
}

#[derive(Message)]
pub enum GameStateEvent {
    SplashComplete,
    PlayRequested,
    MapSelected(MapSelection),
    GameComplete,
}

/// Single registration point for the game.
///
/// `main` adds only this plugin; everything else (windowing, physics, game
/// states, and all feature plugins) is wired up here so the entry point stays
/// a one-liner and the plugin set lives in one readable place.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window { title: "Logs Of War".into(), ..default() }),
                ..default()
            }),
            PhysicsPlugins::default(),
            game_flow_plugin,
            splash_plugin,
            menu_plugin,
            briefing_plugin,
            MapBattlePlugin,
            PhysicsBasePlugin,
            CharacterControllerPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera);
    }
}

fn game_flow_plugin(app: &mut App) {
    app.add_message::<GameStateEvent>().add_systems(Update, handle_game_flow_events);
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
                game_state.set(GameState::Briefing);
            }
            GameStateEvent::MapSelected(map) => {
                *map_selection = *map;
                info!("Building map: {:?}", map);
                game_state.set(GameState::Battle);
            }
            GameStateEvent::GameComplete => {
                game_state.set(GameState::Menu);
            }
        }
    }
}
