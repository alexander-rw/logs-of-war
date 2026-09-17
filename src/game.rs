use avian3d::prelude::*;
use bevy::prelude::*;

use crate::battle::battle_plugin;
use crate::briefing::briefing_plugin;
use crate::camera::camera_plugin;
use crate::character::character_plugin;
use crate::maps::maps_plugin;
use crate::menu::menu_plugin;
use crate::physics::physics_plugin;
use crate::splash::splash_plugin;
use crate::ui::ui_plugin;

/// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Briefing,
    Battle,
}

/// Single registration point for the game.
///
/// `main` adds only this plugin; everything else (windowing, physics, game
/// states, and all feature plugins) is wired up here so the entry point stays
/// a one-liner and the plugin set lives in one readable place.
pub fn game_plugin(app: &mut App) {
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Logs Of War".into(), ..default() }),
            ..default()
        }),
        PhysicsPlugins::default(),
        camera_plugin,
        ui_plugin,
        splash_plugin,
        menu_plugin,
        briefing_plugin,
        maps_plugin,
        battle_plugin,
        physics_plugin,
        character_plugin,
    ))
    .init_state::<GameState>();
}
