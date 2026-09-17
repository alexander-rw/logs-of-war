//! The Testing Area map: flat ground with one soldier a side.
//!
//! Only available in debug builds (`cargo build`, not `cargo build --release`).

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::battle::teams::{SpawnConfig, TeamConfig, TeamId};
use crate::maps::{Map, MapSelection, terrain_material};

/// Total width and depth of the ground slab in world units.
const SIZE: f32 = 40.0;

/// Thickness of the ground slab in world units.
const THICKNESS: f32 = 0.5;

/// Y position (height) where characters spawn, above the ground so that
/// physics drops them onto the surface.
const SPAWN_HEIGHT: f32 = 3.0;

/// Distance from centre (X=0) where each soldier stands.
const SPAWN_X_OFFSET: f32 = 4.0;

pub struct TestingArea;

impl Map for TestingArea {
    const SELECTION: MapSelection = MapSelection::TestingArea;

    /// Spawns a flat cuboid slab.
    ///
    /// A fixed cuboid rather than a heightmap, so the surface is perfectly
    /// level — useful for isolated unit behaviour testing.
    fn spawn_terrain(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        info!("Building map: {:?}", Self::SELECTION);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(SIZE, THICKNESS, SIZE))),
            MeshMaterial3d(materials.add(terrain_material())),
            Transform::from_translation(Vec3::new(0.0, -THICKNESS / 2.0, 0.0)),
            RigidBody::Static,
            Collider::cuboid(SIZE, THICKNESS, SIZE),
        ));
    }

    /// One soldier per team on flat ground: Blue is keyboard-driven
    /// (WASD + Space), Red stands as a static reference.
    fn teams() -> SpawnConfig {
        SpawnConfig {
            teams: vec![
                TeamConfig {
                    team_id: TeamId::Red,
                    positions: vec![Vec3::new(-SPAWN_X_OFFSET, SPAWN_HEIGHT, 0.0)],
                    player_controlled: false,
                },
                TeamConfig {
                    team_id: TeamId::Blue,
                    positions: vec![Vec3::new(SPAWN_X_OFFSET, SPAWN_HEIGHT, 0.0)],
                    player_controlled: true,
                },
            ],
        }
    }
}
