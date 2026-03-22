use avian3d::prelude::*;
use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use crate::components::game_camera::GameCamera;
use crate::plugins::*;
use crate::resources::game_state::GameState;

fn main() {
    let app_default_plugin = DefaultPlugins
        .set(WindowPlugin { primary_window: Some(Window { title: "Logs Of War".into(), ..default() }), ..default() });

    let plugins = (
        app_default_plugin,
        PhysicsPlugins::default(),
        game_flow::game_flow_plugin,
        splash::splash_plugin,
        menu::menu_plugin,
        map_settings::map_setting_plugin,
        map_battle::MapBattlePlugin,
        physics_base_plugin::PhysicsBasePlugin,
        character_controller::CharacterControllerPlugin
    );

    App::new().add_plugins(plugins).add_systems(Startup, setup).init_state::<GameState>().run();
}

pub fn setup(mut commands: Commands) {
    info_once!("Setup system initialized.");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
}