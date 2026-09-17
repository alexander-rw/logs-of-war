//! The Hills map: procedural sine-wave hills, with four soldiers a side.

use avian3d::prelude::{Collider, RigidBody};
use bevy::mesh::VertexAttributeValues;
use bevy::prelude::*;

use crate::battle::teams::{SpawnConfig, Team, TeamConfig};
use crate::game::GameState;
use crate::maps::{FormationConfig, Map, MapSelection, terrain_material};

/// Total width and depth of the terrain in world units.
/// The terrain extends from -SIZE/2 to +SIZE/2 on both X and Z axes.
const SIZE: f32 = 40.0;

/// Number of quad subdivisions per axis. Higher values give smoother hills
/// but more vertices.
const SUBDIVISIONS: u32 = 32;

/// Maximum hill height amplitude in world units.
const HEIGHT_SCALE: f32 = 1.5;

/// Where each team lines up.
const FORMATION: FormationConfig =
    FormationConfig { spawn_x_offset: 15.0, spawn_height: 3.0, team_size: 4, z_spacing: 3.0 };

pub struct Hills;

impl Map for Hills {
    const SELECTION: MapSelection = MapSelection::Hills;

    /// Spawns the Hills terrain as a heightmap mesh with a physics collider.
    ///
    /// Uses a trimesh collider for accurate physics collision detection. If the
    /// collider cannot be built from the mesh, an error is logged and the
    /// terrain is spawned without physics.
    fn spawn_terrain(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        info!("Building map: {:?}", Self::SELECTION);

        let terrain_mesh = generate_heightmap_mesh(SIZE, SUBDIVISIONS, HEIGHT_SCALE);
        let Some(collider) = Collider::trimesh_from_mesh(&terrain_mesh) else {
            error!("Failed to create trimesh collider — terrain will not have physics");
            return;
        };

        commands.spawn((
            Name::new("Terrain"),
            RigidBody::Static,
            collider,
            Mesh3d(meshes.add(terrain_mesh)),
            MeshMaterial3d(materials.add(terrain_material())),
            DespawnOnExit(GameState::Battle),
        ));
    }

    fn teams() -> SpawnConfig {
        SpawnConfig {
            teams: vec![
                TeamConfig {
                    team: Team::Red,
                    positions: FORMATION.spawn_positions(Team::Red),
                    player_controlled: false,
                },
                TeamConfig {
                    team: Team::Blue,
                    positions: FORMATION.spawn_positions(Team::Blue),
                    player_controlled: false,
                },
            ],
        }
    }
}

/// Generates a heightmap terrain mesh with procedural sine-wave hills.
///
/// Creates a subdivided plane mesh and perturbs vertex Y positions using
/// overlapping sine waves to create gentle, rolling hills.
#[must_use]
fn generate_heightmap_mesh(size: f32, subdivisions: u32, height_scale: f32) -> Mesh {
    // Create a flat plane mesh as the base.
    // Plane3d creates a plane facing upward (Y-up) by default.
    let mut mesh = Plane3d::default().mesh().size(size, size).subdivisions(subdivisions).build();

    // Modify vertex Y positions to create hills. The attribute is only
    // present, and only Float32x3, on a mesh built by `Plane3d`.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heightmap_stays_within_the_height_scale() {
        let mesh = generate_heightmap_mesh(SIZE, SUBDIVISIONS, HEIGHT_SCALE);
        let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("the plane mesh always has Float32x3 positions");
        };

        assert!(!positions.is_empty());
        for pos in positions {
            assert!(pos[1].abs() <= HEIGHT_SCALE);
        }
    }
}
