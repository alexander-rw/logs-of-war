use bevy::prelude::*;

mod components;
mod queries;
mod plugins;
mod resources;

use crate::plugins::hello_plugin::HelloPlugin;

fn main() {
    let app_default_plugin = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logs Of War".into(),
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins(app_default_plugin)
        .add_plugins(HelloPlugin)
        .run();
}
