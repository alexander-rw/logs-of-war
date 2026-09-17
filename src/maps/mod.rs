//! The maps a battle can be fought on.
//!
//! Each [`MapSelection`] variant corresponds to one terrain module:
//! - [`hills`] — procedural sine-wave heightmap (Hills map)
//! - [`testing_area`] — flat cuboid (Testing Area, debug builds only)

pub mod hills;

#[cfg(debug_assertions)]
pub mod testing_area;

use bevy::prelude::*;

use crate::battle::teams::{SpawnConfig, TeamConfig, TeamId};

/// The map selected by the player before a battle.
///
/// Adding a new variant requires:
/// - A new arm in [`MapSelection::generate`] (this file only)
/// - A new arm in [`MapSelection::label`] (this file only)
/// - A new arm in [`MapSelection::all_variants`] (this file only)
/// - A new arm in [`MapSelection::spawn_config`] (this file only)
/// - A new terrain module in `src/maps/`
///
/// `battle/mod.rs` never needs to change.
#[derive(Clone, Copy, PartialEq, Debug, Default, Resource)]
pub enum MapSelection {
    #[default]
    Hills,

    #[cfg(debug_assertions)]
    TestingArea,
}

impl MapSelection {
    /// Returns a human-readable display name for use in the UI dropdown.
    ///
    /// Prefer this over `format!("{:?}", variant)`, which would give
    /// `"TestingArea"` instead of `"Testing Area"`.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Hills => "Hills",
            #[cfg(debug_assertions)]
            Self::TestingArea => "Testing Area",
        }
    }

    /// Returns all variants available in the current build configuration.
    ///
    /// Debug builds include [`MapSelection::TestingArea`]; release builds do not.
    ///
    /// The slice is a compile-time constant — no heap allocation occurs.
    pub fn all_variants() -> &'static [Self] {
        #[cfg(debug_assertions)]
        return &[Self::Hills, Self::TestingArea];
        #[cfg(not(debug_assertions))]
        return &[Self::Hills];
    }

    /// Returns the spawn configuration for this map: which teams exist and
    /// where each player spawns. `battle/mod.rs` inserts this as a resource
    /// before `spawn_teams` runs.
    pub fn spawn_config(&self, terrain: &TerrainConfig) -> SpawnConfig {
        match self {
            Self::Hills => SpawnConfig {
                teams: vec![
                    TeamConfig {
                        team_id: TeamId::Red,
                        positions: terrain.spawn_positions(TeamId::Red),
                        player_controlled: false,
                    },
                    TeamConfig {
                        team_id: TeamId::Blue,
                        positions: terrain.spawn_positions(TeamId::Blue),
                        player_controlled: false,
                    },
                ],
            },
            // One character per team on flat ground: Blue is keyboard-driven
            // (WASD + Space), Red stands as a static reference.
            #[cfg(debug_assertions)]
            Self::TestingArea => SpawnConfig {
                teams: vec![
                    TeamConfig {
                        team_id: TeamId::Red,
                        positions: vec![Vec3::new(-4.0, terrain.spawn_height, 0.0)],
                        player_controlled: false,
                    },
                    TeamConfig {
                        team_id: TeamId::Blue,
                        positions: vec![Vec3::new(4.0, terrain.spawn_height, 0.0)],
                        player_controlled: true,
                    },
                ],
            },
        }
    }

    /// Spawns the terrain for the selected map.
    ///
    /// This is the single dispatch point for terrain generation — `battle/mod.rs`
    /// calls this and never needs to know which variant is active.
    pub fn generate(
        &self,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        config: Res<TerrainConfig>,
    ) {
        match self {
            Self::Hills => hills::spawn_terrain(commands, meshes, materials, config),
            #[cfg(debug_assertions)]
            Self::TestingArea => testing_area::spawn_terrain_flat(commands, meshes, materials),
        }
    }
}

/// Configuration for terrain generation and team spawn positioning.
///
/// This resource centralizes all world geometry parameters, making spawn
/// locations tied to the terrain dimensions rather than hardcoded values.
#[derive(Resource, Clone, Debug)]
pub struct TerrainConfig {
    /// Total width and depth of the terrain in world units.
    /// The terrain extends from -size/2 to +size/2 on both X and Z axes.
    pub size: f32,

    /// Number of quad subdivisions per axis for mesh smoothness.
    /// Higher values create smoother hills but more vertices.
    pub subdivisions: u32,

    /// Maximum hill height amplitude in world units.
    pub height_scale: f32,

    /// Distance from center (X=0) where teams spawn.
    /// Red team spawns at -spawn_x_offset, Blue at +spawn_x_offset.
    pub spawn_x_offset: f32,

    /// Y position (height) where characters spawn.
    /// Should be above terrain to allow physics to drop them onto surface.
    pub spawn_height: f32,

    /// Number of characters per team.
    pub team_size: usize,

    /// Spacing between characters along the Z axis.
    pub z_spacing: f32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            size: 40.0,
            subdivisions: 32,
            height_scale: 1.5,
            spawn_x_offset: 15.0,
            spawn_height: 3.0,
            team_size: 4,
            z_spacing: 3.0,
        }
    }
}

impl TerrainConfig {
    /// Returns spawn positions for all characters on a team.
    ///
    /// Positions are centered along the Z axis and spaced according to
    /// `z_spacing`. Red team spawns on negative X, Blue on positive X.
    #[must_use]
    pub fn spawn_positions(&self, team: TeamId) -> Vec<Vec3> {
        let mut positions = Vec::with_capacity(self.team_size);

        // X position: negative for Red, positive for Blue
        let x = match team {
            TeamId::Red => -self.spawn_x_offset,
            TeamId::Blue => self.spawn_x_offset,
        };

        // Calculate Z offset to center the team formation
        // For 4 characters with 3.0 spacing: z_start = -4.5
        let z_start = -((self.team_size as f32 - 1.0) * self.z_spacing) / 2.0;

        for i in 0..self.team_size {
            let z = z_start + (i as f32) * self.z_spacing;
            positions.push(Vec3::new(x, self.spawn_height, z));
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let config = TerrainConfig::default();
        assert_eq!(config.size, 40.0);
        assert_eq!(config.team_size, 4);
        assert_eq!(config.spawn_x_offset, 15.0);
    }

    #[test]
    fn spawn_positions_returns_correct_count() {
        let config = TerrainConfig::default();
        let red_positions = config.spawn_positions(TeamId::Red);
        let blue_positions = config.spawn_positions(TeamId::Blue);

        assert_eq!(red_positions.len(), 4);
        assert_eq!(blue_positions.len(), 4);
    }

    #[test]
    fn red_team_spawns_on_negative_x() {
        let config = TerrainConfig::default();
        let positions = config.spawn_positions(TeamId::Red);

        for pos in positions {
            assert_eq!(pos.x, -15.0);
            assert_eq!(pos.y, 3.0);
        }
    }

    #[test]
    fn blue_team_spawns_on_positive_x() {
        let config = TerrainConfig::default();
        let positions = config.spawn_positions(TeamId::Blue);

        for pos in positions {
            assert_eq!(pos.x, 15.0);
            assert_eq!(pos.y, 3.0);
        }
    }

    #[test]
    fn spawn_positions_are_centered_on_z() {
        let config = TerrainConfig::default();
        let positions = config.spawn_positions(TeamId::Red);

        // With 4 characters and 3.0 spacing: -4.5, -1.5, +1.5, +4.5
        let z_values: Vec<f32> = positions.iter().map(|p| p.z).collect();
        assert_eq!(z_values, vec![-4.5, -1.5, 1.5, 4.5]);
    }
}
