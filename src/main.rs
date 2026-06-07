use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use crate::plugins::game::GamePlugin;

fn main() {
    App::new().add_plugins(GamePlugin).run();
}
