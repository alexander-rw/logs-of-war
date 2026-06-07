//! Heightmap terrain generation and spawning for the Hills map.
//!
//! Creates procedural sine-wave hills using a subdivided plane mesh
//! with perturbed vertex Y positions.

use avian3d::prelude::{Collider, RigidBody};
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use crate::resources::game_state::GameState;
use crate::resources::terrain_config::TerrainConfig;

/// Generates a heightmap terrain mesh with procedural sine-wave hills.
///
/// Creates a subdivided plane mesh and perturbs vertex Y positions using
/// overlapping sine waves to create gentle, rolling hills.
///
/// # Arguments
///
/// * `size` - Total width and depth of the terrain in world units
/// * `subdivisions` - Number of quad subdivisions per axis (higher = smoother)
/// * `height_scale` - Maximum hill height amplitude
///
/// # Returns
///
/// A [`Mesh`] with modified vertex positions and recalculated normals.
///
/// # Examples
///
/// ```ignore
/// let terrain_mesh = generate_heightmap_mesh(40.0, 32, 1.5);
/// ```
#[must_use]
pub fn generate_heightmap_mesh(size: f32, subdivisions: u32, height_scale: f32) -> Mesh {
    // Create a flat plane mesh as the base.
    // Plane3d creates a plane facing upward (Y-up) by default.
    let mut mesh = Plane3d::default().mesh().size(size, size).subdivisions(subdivisions).build();

    // Modify vertex Y positions to create hills.
    // `if let Some(...)` safely unwraps an Option — like a null check in C# but
    // combined with a type cast. If the attribute exists and matches Float32x3,
    // we get mutable access to the positions array.
    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            // Combine two sine waves at different frequencies for natural-looking hills.
            // pos[0] is X, pos[2] is Z (Y is up in Bevy).
            let height = (pos[0] * 0.15).sin() * (pos[2] * 0.1).cos() * height_scale;
            pos[1] = height;
        }
    }

    // Recalculate normals after modifying vertex positions so lighting looks correct.
    mesh.compute_normals();

    mesh
}

/// Spawns the Hills terrain as a heightmap mesh with a physics collider.
///
/// Uses [`TerrainConfig`] resource for terrain dimensions and properties.
/// Creates a trimesh collider for accurate physics collision detection.
///
/// # Arguments
///
/// * `commands` - Bevy command buffer for spawning entities
/// * `meshes` - Asset storage for meshes
/// * `materials` - Asset storage for standard materials
/// * `config` - Terrain configuration resource
///
/// # Notes
///
/// If the trimesh collider cannot be built from the mesh, an error is logged
/// and the terrain is spawned without a physics collider.
// Note for C# developers: Bevy "systems" are plain functions whose parameters
// are automatically injected by the ECS at runtime based on their types.
// `Commands` spawns entities, `ResMut<Assets<T>>` gives mutable access to
// asset storage, and `Res<T>` gives read-only access to a singleton resource.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>,
) {
    let terrain_mesh = generate_heightmap_mesh(config.size, config.subdivisions, config.height_scale);
    let Some(collider) = Collider::trimesh_from_mesh(&terrain_mesh) else {
        error!("Failed to create trimesh collider — terrain will not have physics");
        return;
    };

    let mesh_handle = meshes.add(terrain_mesh);
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    commands.spawn((
        Name::new("Terrain"),
        RigidBody::Static,
        collider,
        Mesh3d(mesh_handle),
        MeshMaterial3d(terrain_material),
        DespawnOnExit(GameState::Game),
    ));
}
