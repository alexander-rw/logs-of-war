//! Flat terrain spawning for the Testing Area map.
//!
//! Only available in debug builds (`cargo build`, not `cargo build --release`).

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

/// Spawns a flat cuboid terrain for testing purposes.
///
/// Uses a fixed 40×0.5×40 cuboid rather than a heightmap so the surface
/// is perfectly level — useful for isolated unit behaviour testing.
///
/// # Arguments
///
/// * `commands` - Bevy command buffer for spawning entities
/// * `meshes` - Asset storage for meshes
/// * `materials` - Asset storage for standard materials
#[cfg(debug_assertions)]
pub fn spawn_terrain_flat(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let terrain_mesh = meshes.add(Cuboid::new(40.0, 0.5, 40.0));
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn((
        Mesh3d(terrain_mesh),
        MeshMaterial3d(terrain_material),
        Transform::from_translation(Vec3::new(0.0, -0.25, 0.0)),
        RigidBody::Static,
        Collider::cuboid(40.0, 0.5, 40.0),
    ));
}
