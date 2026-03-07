//! Terrain generation and spawning systems.
//!
//! This module provides procedural heightmap terrain generation with
//! sine-wave hills for visual interest.

use avian3d::prelude::{Collider, RigidBody};
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use crate::resources::game_state::GameState;

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
// Note for Python developers: This function takes ownership of nothing and
// returns a new Mesh. The mesh is created on the heap and returned by value,
// which in Rust means ownership is transferred to the caller.
#[must_use]
pub fn generate_heightmap_mesh(size: f32, subdivisions: u32, height_scale: f32) -> Mesh {
    // Create a flat plane mesh as the base
    // Plane3d creates a plane facing upward (Y-up) by default
    let mut mesh = Plane3d::default().mesh().size(size, size).subdivisions(subdivisions).build();

    // Modify vertex Y positions to create hills
    // Note for Python developers: `if let Some(...)` is Rust's way of safely
    // unwrapping an Option. If the attribute exists and matches the expected
    // type, we get mutable access to the positions array.
    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        for pos in positions.iter_mut() {
            // Combine two sine waves at different frequencies for natural-looking hills
            // pos[0] is X, pos[2] is Z (Y is up in Bevy)
            let height = (pos[0] * 0.15).sin() * (pos[2] * 0.1).cos() * height_scale;
            pos[1] = height;
        }
    }

    // Recalculate normals after modifying vertex positions
    // This ensures lighting looks correct on the hills
    mesh.compute_normals();

    mesh
}

/// Spawns the game terrain as a heightmap with physics collider.
///
/// Creates a 40x40 unit terrain with gentle hills and a trimesh collider
/// for accurate physics collision detection.
///
/// # Arguments
///
/// * `commands` - Bevy command buffer for spawning entities
/// * `meshes` - Asset storage for meshes
/// * `materials` - Asset storage for materials
///
/// # Panics
///
/// Panics if the trimesh collider cannot be created from the mesh.
// Note for Python developers: This is a Bevy "system" function. The parameters
// are automatically injected by Bevy's ECS at runtime based on their types.
// `Commands` lets us spawn entities, `ResMut<Assets<T>>` gives mutable access
// to asset storage.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Generate the heightmap terrain mesh
    // 40x40 units, 32 subdivisions per axis, 1.5 unit max hill height
    let terrain_mesh = generate_heightmap_mesh(40.0, 32, 1.5);

    // Create a trimesh collider from the mesh for accurate physics
    // Note for Python developers: `expect()` is like Python's assert - it
    // unwraps the Result or panics with the given message if it's an error.
    let collider =
        Collider::trimesh_from_mesh(&terrain_mesh).expect("Failed to create trimesh collider from terrain mesh");

    // Add the mesh to the asset store and get a handle
    let mesh_handle = meshes.add(terrain_mesh);

    // Create a grass-green material for the terrain
    let terrain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    // Spawn the terrain entity with all components
    commands.spawn((
        Name::new("Terrain"),
        RigidBody::Static,
        collider,
        Mesh3d(mesh_handle),
        MeshMaterial3d(terrain_material),
        DespawnOnExit(GameState::Game),
    ));
}
