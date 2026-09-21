//! The maps a battle can be fought on.
//!
//! A map is a type that implements [`Map`]. To add one:
//! - Write the `impl Map` in a new module here
//! - Add a [`MapSelection`] variant, with its [`MapSelection::label`] arm and
//!   its [`MapSelection::all_variants`] entry
//! - Add a [`map_plugin`] line to [`maps_plugin`]
//!
//! The battle code never changes.

pub mod hills;

#[cfg(debug_assertions)]
pub mod testing_area;

use bevy::prelude::*;

use crate::battle::teams::{SpawnConfig, Team, spawn_teams};
use crate::game::GameState;

/// One battlefield: its terrain and the teams that start on it.
pub trait Map: Send + Sync + 'static {
    /// The [`MapSelection`] variant that chooses this map.
    const SELECTION: MapSelection;

    /// Spawns the terrain of this map.
    fn spawn_terrain(commands: Commands, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<StandardMaterial>>);

    /// Returns which teams start on this map, and where.
    fn teams() -> SpawnConfig;
}

/// Registers one map. Its systems run on entering a battle, but only when the
/// player selected this map.
pub fn map_plugin<M: Map>(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Battle),
        (M::spawn_terrain, spawn_teams::<M>).run_if(resource_equals(M::SELECTION)),
    );
}

/// Registers every map, and the selection that picks between them.
///
/// The selection is initialised once here, so it survives a return to the
/// briefing screen.
pub fn maps_plugin(app: &mut App) {
    app.init_resource::<MapSelection>().add_plugins(map_plugin::<hills::Hills>);

    #[cfg(debug_assertions)]
    app.add_plugins(map_plugin::<testing_area::TestingArea>);
}

/// The map selected by the player before a battle.
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
}

/// The material every map's terrain uses.
pub fn terrain_material() -> StandardMaterial {
    StandardMaterial { base_color: Color::srgb(0.3, 0.5, 0.2), perceptual_roughness: 0.9, ..default() }
}

/// A line of spawn positions for one team, centred on the Z axis.
///
/// A map that starts its teams in a line builds its positions with this; a map
/// with only a handful of soldiers writes their positions out instead.
pub struct FormationConfig {
    /// Distance from centre (X=0) where the teams spawn.
    /// Red spawns at -spawn_x_offset, Blue at +spawn_x_offset.
    pub spawn_x_offset: f32,

    /// Y position (height) where soldiers spawn.
    /// Should be above terrain to allow physics to drop them onto surface.
    pub spawn_height: f32,

    /// Number of soldiers per team.
    pub team_size: usize,

    /// Spacing between soldiers along the Z axis.
    pub z_spacing: f32,
}

impl FormationConfig {
    /// Returns spawn positions for all soldiers on a team.
    ///
    /// Positions are centred along the Z axis and spaced according to
    /// `z_spacing`. Red team spawns on negative X, Blue on positive X.
    #[must_use]
    pub fn spawn_positions(&self, team: Team) -> Vec<Vec3> {
        let mut positions = Vec::with_capacity(self.team_size);

        // X position: negative for Red, positive for Blue
        let x = match team {
            Team::Red => -self.spawn_x_offset,
            Team::Blue => self.spawn_x_offset,
        };

        // Calculate Z offset to center the team formation
        // For 4 soldiers with 3.0 spacing: z_start = -4.5
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

    const FORMATION: FormationConfig =
        FormationConfig { spawn_x_offset: 15.0, spawn_height: 3.0, team_size: 4, z_spacing: 3.0 };

    #[test]
    fn spawn_positions_returns_one_position_per_soldier() {
        assert_eq!(FORMATION.spawn_positions(Team::Red).len(), 4);
        assert_eq!(FORMATION.spawn_positions(Team::Blue).len(), 4);
    }

    #[test]
    fn red_team_spawns_on_negative_x() {
        for pos in FORMATION.spawn_positions(Team::Red) {
            assert_eq!(pos.x, -15.0);
            assert_eq!(pos.y, 3.0);
        }
    }

    #[test]
    fn blue_team_spawns_on_positive_x() {
        for pos in FORMATION.spawn_positions(Team::Blue) {
            assert_eq!(pos.x, 15.0);
            assert_eq!(pos.y, 3.0);
        }
    }

    #[test]
    fn spawn_positions_are_centered_on_z() {
        // With 4 soldiers and 3.0 spacing: -4.5, -1.5, +1.5, +4.5
        let positions = FORMATION.spawn_positions(Team::Red);
        let z_values: Vec<f32> = positions.iter().map(|p| p.z).collect();
        assert_eq!(z_values, vec![-4.5, -1.5, 1.5, 4.5]);
    }
}
