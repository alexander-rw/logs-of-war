use bevy::prelude::*;

mod battle;
mod briefing;
mod camera;
mod character;
mod game;
mod maps;
mod menu;
mod physics;
mod splash;
mod ui;

use crate::game::game_plugin;

fn main() {
    App::new().add_plugins(game_plugin).run();
}
