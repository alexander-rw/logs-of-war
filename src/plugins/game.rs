use avian3d::prelude::*;
use bevy::prelude::*;

use crate::components::game_camera::GameCamera;
use crate::plugins::{
    character_controller::CharacterControllerPlugin, game_flow::game_flow_plugin, map_battle::MapBattlePlugin,
    map_settings::map_setting_plugin, menu::menu_plugin, physics_base_plugin::PhysicsBasePlugin, splash::splash_plugin,
};
use crate::resources::game_state::GameState;

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
            map_setting_plugin,
            MapBattlePlugin,
            PhysicsBasePlugin,
            CharacterControllerPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera);
    }
}

/// Spawns the single game camera.
fn setup_camera(mut commands: Commands) {
    info_once!("Setup system initialized.");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
}
