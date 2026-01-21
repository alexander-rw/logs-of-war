use bevy::prelude::*;

mod components;

use crate::components::name::Name;
use crate::components::person::Person;

fn hello_world_system() {
    println!("hello world");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, hello_world_system)
        .run();
}

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
