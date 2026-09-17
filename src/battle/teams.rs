//! Teams and the system that spawns their soldiers.

use avian3d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJumpConfig, TnuaBuiltinWalkConfig};
use bevy_tnua::prelude::{TnuaConfig, TnuaController};
use bevy_tnua_avian3d::prelude::TnuaAvian3dSensorShape;

use crate::character::controller::{
    BODY_HALF_HEIGHT, BODY_RADIUS, ControlScheme, ControlSchemeConfig, FLOAT_HEIGHT, JUMP_HEIGHT, PlayerControlled,
    SENSOR_RADIUS,
};
use crate::character::{Soldier, random_name};
use crate::game::GameState;
use crate::maps::Map;

/// One of the two sides in a battle. Every soldier belongs to exactly one.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    /// The red team, spawning on the negative X side of the map.
    Red,
    /// The blue team, spawning on the positive X side of the map.
    Blue,
}

impl Team {
    #[must_use]
    pub fn color(&self) -> Color {
        match self {
            Team::Red => Color::srgb(0.85, 0.2, 0.2),  // Crimson red
            Team::Blue => Color::srgb(0.2, 0.4, 0.85), // Royal blue
        }
    }

    /// Returns a human-readable name for the team.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Team::Red => "Red Team",
            Team::Blue => "Blue Team",
        }
    }
}

/// Spawn positions and team assignment for a single team.
pub struct TeamConfig {
    pub team: Team,
    pub positions: Vec<Vec3>,
    /// When true, this team's characters receive the keyboard-driven
    /// [`PlayerControlled`] marker.
    pub player_controlled: bool,
}

/// Which teams start on a map, and where. Returned by [`Map::teams`].
pub struct SpawnConfig {
    pub teams: Vec<TeamConfig>,
}

/// Spawns every team's characters from the map's [`SpawnConfig`].
///
/// All characters share one body mesh and one Tnua control-scheme config asset,
/// and each team shares a single material. The team flagged
/// [`TeamConfig::player_controlled`] also gets the [`PlayerControlled`] marker
/// so keyboard input drives it.
pub fn spawn_teams<M: Map>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scheme_configs: ResMut<Assets<ControlSchemeConfig>>,
) {
    let config = M::teams();

    // Cylinder body shared by every soldier.
    let body_mesh = meshes.add(Cylinder { radius: BODY_RADIUS, half_height: BODY_HALF_HEIGHT });

    // One control-scheme config asset tunes the basis and jump for all characters.
    let scheme_config = scheme_configs.add(ControlSchemeConfig {
        basis: TnuaBuiltinWalkConfig { float_height: FLOAT_HEIGHT, ..default() },
        jump: TnuaBuiltinJumpConfig { height: JUMP_HEIGHT, ..default() },
    });

    for team in &config.teams {
        let material = materials.add(StandardMaterial { base_color: team.team.color(), ..default() });

        for &position in &team.positions {
            let mut soldier = commands.spawn((
                Name::new(format!("{} ({})", random_name(), team.team.name())),
                Mesh3d(body_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position),
                RigidBody::Dynamic,
                Collider::capsule(BODY_RADIUS, BODY_HALF_HEIGHT),
                LockedAxes::ROTATION_LOCKED,
                Soldier,
                team.team,
                DespawnOnExit(GameState::Battle),
                (
                    TnuaController::<ControlScheme>::default(),
                    TnuaConfig::<ControlScheme>(scheme_config.clone()),
                    TnuaAvian3dSensorShape(Collider::cylinder(SENSOR_RADIUS, 0.0)),
                ),
            ));

            if team.player_controlled {
                soldier.insert(PlayerControlled);
            }
        }
    }
}
