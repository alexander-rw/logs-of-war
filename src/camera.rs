use bevy::prelude::*;

/// Marks the single camera that views the battlefield.
#[derive(Component)]
pub struct GameCamera;

/// Spawns the single game camera.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
}
