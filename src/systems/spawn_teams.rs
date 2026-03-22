//! Team spawning system.
//!
//! This module provides the system for spawning log soldier characters
//! for each team at their designated positions.

use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::components::team::{Team, TeamId};
use crate::components::character::TreeCharacter;
use crate::resources::game_state::GameState;
use crate::resources::terrain_config::TerrainConfig;

/// Spawns a soldier entity with mesh, collider, and components.
///
/// Creates a character with:
/// - Cylinder body (radius 0.3, height 1.2) for the log torso
/// - Sphere head (radius 0.25) as a child entity
/// - Capsule physics collider
/// - Team-colored material
///
/// # Arguments
///
/// * `commands` - Bevy command buffer
/// * `meshes` - Mesh asset store
/// * `materials` - Material asset store
/// * `position` - World position to spawn at
/// * `team_id` - Team assignment
///
/// # Returns
///
/// The spawned entity ID.
fn spawn_tree_soldier(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    team_id: TeamId,
) {
    // Create meshes for body and head
    let c = Cylinder::new(0.3, 1.2);
    let body_mesh = meshes.add(c);

    // Create team-colored material
    let material = materials.add(StandardMaterial { base_color: team_id.color(), ..default() });

    // Spawn the body as parent entity with physics
    commands
        .spawn((
            Name::new(format!("{} Soldier", team_id.name())),
            Mesh3d(body_mesh),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(position),
            RigidBody::Dynamic,
            Collider::capsule(c.radius, c.half_height),
            TreeCharacter::default(),
            Team { id: team_id },
            DespawnOnExit(GameState::Game),
        ));
}

/// Spawns all teams with their characters at configured positions.
///
/// Reads spawn positions from [`TerrainConfig`] and creates log soldiers
/// for both Red and Blue teams.
///
/// # Arguments
///
/// * `commands` - Bevy command buffer
/// * `meshes` - Mesh asset store
/// * `materials` - Material asset store
/// * `config` - Terrain configuration with spawn positions
pub fn spawn_teams(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>,
) {
    // Spawn both teams
    for team_id in [TeamId::Red, TeamId::Blue] {
        // Get spawn positions for this team from config
        let positions = config.spawn_positions(team_id);

        for position in positions {
            spawn_tree_soldier(&mut commands, &mut meshes, &mut materials, position, team_id);
        }
    }
}
