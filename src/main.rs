use avian3d::prelude::*;
use bevy::prelude::*;

mod components;
mod log_character;
mod plugins;
mod resources;
mod setup;
mod systems;

use crate::plugins::*;
use crate::resources::game_state::GameState;

fn main() {
    let app_default_plugin = DefaultPlugins
        .set(WindowPlugin { primary_window: Some(Window { title: "Logs Of War".into(), ..default() }), ..default() });

    let app_plugins = (app_default_plugin, PhysicsPlugins::default());

    App::new()
        .add_plugins(app_plugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup::setup_system)
         // Adds the plugins for each state
        .add_plugins((splash::splash_plugin, menu::menu_plugin, logs_of_war::LogsOfWarPlugin))
        .run();
}
