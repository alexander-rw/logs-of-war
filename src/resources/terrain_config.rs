//! Terrain configuration resource.
//!
//! This module provides the [`TerrainConfig`] resource which defines
//! world geometry parameters and team spawn locations.

use bevy::prelude::{Resource, Vec3};

use crate::components::team::TeamId;

/// Configuration for terrain generation and team spawn positioning.
///
/// This resource centralizes all world geometry parameters, making spawn
/// locations tied to the terrain dimensions rather than hardcoded values.
///
/// # Examples
///
/// ```ignore
/// let config = TerrainConfig::default();
/// let red_spawns = config.spawn_positions(TeamId::Red);
/// let blue_spawns = config.spawn_positions(TeamId::Blue);
/// ```
// Note for Python developers: `#[derive(Resource)]` makes this struct
// usable as a Bevy ECS resource, which is like a singleton that systems
// can access via `Res<TerrainConfig>` or `ResMut<TerrainConfig>`.
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
    ///
    /// # Arguments
    ///
    /// * `team` - The team to generate spawn positions for
    ///
    /// # Returns
    ///
    /// A vector of world positions where characters should spawn.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let config = TerrainConfig::default();
    /// let positions = config.spawn_positions(TeamId::Red);
    /// // Returns 4 positions at X = -15
    /// ```
    // Note for Python developers: `Vec<Vec3>` is like `list[tuple[float, float, float]]`.
    // We allocate with `with_capacity` since we know the exact size, avoiding
    // reallocations as we push elements.
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
