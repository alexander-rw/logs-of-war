use avian3d::prelude::*;
use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use crate::plugins::*;
use crate::resources::game_state::GameState;

fn main() {
    let app_default_plugin = DefaultPlugins
        .set(WindowPlugin { primary_window: Some(Window { title: "Logs Of War".into(), ..default() }), ..default() });

    let app_plugins = (
        app_default_plugin,
        PhysicsPlugins::default(),
        game_flow::game_flow_plugin,
        splash::splash_plugin,
        menu::menu_plugin,
        map_settings::map_setting_plugin,
        map_battle::MapBattlePlugin,
    );

    App::new().add_plugins(app_plugins).add_systems(Startup, systems::setup::setup_system).init_state::<GameState>().run();
}
