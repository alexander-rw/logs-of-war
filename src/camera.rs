use bevy::prelude::*;

use crate::game::GameState;

/// Marks the single camera that views the battlefield.
#[derive(Component)]
pub struct GameCamera;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(Startup, setup_camera).add_systems(OnEnter(GameState::Battle), update_camera);
}

/// Spawns the single game camera.
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-8.5, 14.5, 19.0).looking_at(Vec3::ZERO, Vec3::Y),
        GameCamera,
    ));
}

/// Positions the camera to view the entire battlefield from an elevated angle.
fn update_camera(mut q: Query<&mut Transform, (With<Camera3d>, With<GameCamera>)>) {
    if let Ok(mut transform) = q.single_mut() {
        transform.translation = Vec3::new(0.0, 20.0, 25.0);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
