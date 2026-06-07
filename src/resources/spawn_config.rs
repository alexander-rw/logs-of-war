use bevy::prelude::*;

use crate::components::team::TeamId;

/// Spawn positions and team assignment for a single team.
pub struct TeamConfig {
    pub team_id: TeamId,
    pub positions: Vec<Vec3>,
    /// When true, this team's characters receive the keyboard-driven
    /// [`crate::components::controller::PlayerControlled`] marker.
    pub player_controlled: bool,
}

/// Per-map spawn configuration, built from [`crate::resources::map_selection::MapSelection`]
/// before `OnEnter(GameState::Game)` runs.
///
/// This is the runtime authority for who spawns where. `spawn_teams` reads this
/// resource rather than deriving positions from [`crate::resources::terrain_config::TerrainConfig`].
#[derive(Resource)]
pub struct SpawnConfig {
    pub teams: Vec<TeamConfig>,
}
