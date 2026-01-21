use bevy::{camera::Camera2d, ecs::system::Commands};


pub fn setup_system(mut commands: Commands) {
    println!("Setup system initialized.");
    commands.spawn(Camera2d);
}