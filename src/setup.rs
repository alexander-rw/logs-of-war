use bevy::prelude::*;

use crate::components::game_camera::GameCamera;

pub fn setup_system(mut commands: Commands) {
    println!("Setup system initialized.");
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
}
