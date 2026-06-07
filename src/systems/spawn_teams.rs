//! Team spawning system.
//!
//! This module provides the system for spawning log soldier characters
//! for each team at their designated positions.

use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua::prelude::{TnuaConfig, TnuaController};
use bevy_tnua_avian3d::prelude::TnuaAvian3dSensorShape;

use crate::components::character::TreeCharacter;
use crate::components::controller::{ControlScheme, ControlSchemeConfig, FLOAT_HEIGHT, JUMP_HEIGHT, PlayerControlled};
use crate::components::team::Team;
use crate::resources::game_state::GameState;
use crate::resources::spawn_config::SpawnConfig;

/// Spawns every team's characters from [`SpawnConfig`].
///
/// All characters share one body mesh and one Tnua control-scheme config asset,
/// and each team shares a single material. The team flagged
/// [`crate::resources::spawn_config::TeamConfig::player_controlled`] also gets
/// the [`PlayerControlled`] marker so keyboard input drives it.
pub fn spawn_teams(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scheme_configs: ResMut<Assets<ControlSchemeConfig>>,
    config: Res<SpawnConfig>,
) {
    // Cylinder body shared by every soldier; the capsule collider matches it.
    let body = Cylinder::new(0.3, 1.2);
    let body_mesh = meshes.add(body);

    // One control-scheme config asset tunes the basis and jump for all characters.
    let scheme_config = scheme_configs.add(ControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig { float_height: FLOAT_HEIGHT, ..default() },
        jump: TnuaBuiltinJumpConfig { height: JUMP_HEIGHT, ..default() },
    });

    for team in &config.teams {
        let material = materials.add(StandardMaterial { base_color: team.team_id.color(), ..default() });

        for &position in &team.positions {
            let mut soldier = commands.spawn((
                Name::new(format!("{} Soldier", team.team_id.name())),
                Mesh3d(body_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position),
                RigidBody::Dynamic,
                Collider::capsule(body.radius, body.half_height),
                LockedAxes::ROTATION_LOCKED,
                TreeCharacter::default(),
                Team { id: team.team_id },
                DespawnOnExit(GameState::Game),
                (
                    TnuaController::<ControlScheme>::default(),
                    TnuaConfig::<ControlScheme>(scheme_config.clone()),
                    TnuaAvian3dSensorShape(Collider::cylinder(body.radius - 0.01, 0.0)),
                ),
            ));

            if team.player_controlled {
                soldier.insert(PlayerControlled);
            }
        }
    }
}
