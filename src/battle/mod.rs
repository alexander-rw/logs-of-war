pub mod teams;

use bevy::prelude::*;

use crate::battle::teams::spawn_teams;
use crate::character::despawn_on_zero_health;
use crate::game::{GameState, GameStateEvent};
use crate::maps::{MapSelection, TerrainConfig};

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

pub fn battle_plugin(app: &mut App) {
    app.insert_resource(TerrainConfig::default());
    app.insert_resource(MapSelection::default());

    app.add_systems(
        OnEnter(GameState::Battle),
        (setup_spawn_config, (spawn_terrain_for_selection, spawn_teams, map_battle_setup).after(setup_spawn_config)),
    )
    .add_systems(Update, countdown.run_if(in_state(GameState::Battle)))
    .add_systems(FixedUpdate, despawn_on_zero_health);
}

/// Builds and inserts [`crate::battle::teams::SpawnConfig`] from the active [`MapSelection`].
///
/// Must run before [`spawn_teams`] in `OnEnter(GameState::Battle)`.
fn setup_spawn_config(mut commands: Commands, selection: Res<MapSelection>, terrain: Res<TerrainConfig>) {
    commands.insert_resource(selection.spawn_config(&terrain));
}

/// Spawns the terrain for the active [`MapSelection`].
///
/// Dispatch lives in [`MapSelection::generate`] — this system never needs to
/// change when new map variants are added.
fn spawn_terrain_for_selection(
    selection: Res<MapSelection>,
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    config: Res<TerrainConfig>,
) {
    selection.generate(commands, meshes, materials, config);
}

/// Sets up game lighting and timer.
fn map_battle_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Battle),
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.insert_resource(GlobalAmbientLight { brightness: 160.0, ..default() });

    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(mut game_state_writer: MessageWriter<GameStateEvent>, mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if timer.tick(time.delta()).just_finished() {
        game_state_writer.write(GameStateEvent::GameComplete);
    }
}
