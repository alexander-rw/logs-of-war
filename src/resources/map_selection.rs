use bevy::prelude::*;

use crate::components::team::TeamId;
use crate::resources::spawn_config::{SpawnConfig, TeamConfig};
use crate::resources::terrain_config::TerrainConfig;
use crate::systems::terrain::{spawn_terrain, spawn_terrain_flat};

/// The map selected by the player before a battle.
///
/// Each variant corresponds to a terrain generator in `src/systems/terrain/`.
/// Adding a new variant requires:
/// - A new arm in [`MapSelection::generate`] (this file only)
/// - A new arm in [`MapSelection::label`] (this file only)
/// - A new arm in [`MapSelection::all_variants`] (this file only)
/// - A new arm in [`MapSelection::spawn_config`] (this file only)
/// - A new terrain module in `src/systems/terrain/`
///
/// `map_battle.rs` never needs to change.
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
    /// This is the Rust equivalent of C#'s `Enum.GetValues<T>()`. The slice
    /// is a compile-time constant — no heap allocation occurs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for &variant in MapSelection::all_variants() {
    ///     println!("{}", variant.label());
    /// }
    /// ```
    pub fn all_variants() -> &'static [Self] {
        // `#[cfg(...)]` on array elements is valid Rust — the compiler includes
        // or excludes that element at compile time, like C#'s `#if DEBUG`.
        #[cfg(debug_assertions)]
        return &[Self::Hills, Self::TestingArea];
        #[cfg(not(debug_assertions))]
        return &[Self::Hills];
    }

    /// Returns the spawn configuration for this map: which teams exist and
    /// where each player spawns. `map_battle.rs` inserts this as a resource
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
            #[cfg(debug_assertions)]
            Self::TestingArea => SpawnConfig {
                teams: vec![TeamConfig {
                    team_id: TeamId::Blue,
                    positions: vec![Vec3::new(0.0, terrain.spawn_height, 0.0)],
                    player_controlled: false,
                }],
            },
        }
    }

    /// Spawns the terrain for the selected map.
    ///
    /// This is the single dispatch point for terrain generation — `map_battle.rs`
    /// calls this and never needs to know which variant is active.
    ///
    /// # Arguments
    ///
    /// * `commands` - Bevy command buffer
    /// * `meshes` - Mesh asset store
    /// * `materials` - Material asset store
    /// * `config` - Terrain configuration resource
    pub fn generate(
        &self,
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
        config: Res<TerrainConfig>,
    ) {
        match self {
            Self::Hills => spawn_terrain(commands, meshes, materials, config),
            #[cfg(debug_assertions)]
            Self::TestingArea => spawn_terrain_flat(commands, meshes, materials),
        }
    }
}
