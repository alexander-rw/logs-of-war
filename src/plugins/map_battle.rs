use bevy::{
    app::Update,
    ecs::{
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Res, ResMut},
    },
    prelude::*,
    state::{
        condition::in_state,
        state::OnEnter,
    },
    time::{Time, Timer, TimerMode},
};

use crate::{
    components::game_camera::GameCamera,
    resources::game_state::{GameState, GameStateEvent},
    resources::map_selection::MapSelection,
    resources::terrain_config::TerrainConfig,
    systems::despawn_entities::despawn_on_zero_health,
    systems::spawn_teams::spawn_teams,
};

pub struct MapBattlePlugin;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for MapBattlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainConfig::default());
        app.insert_resource(MapSelection::default());

        app
            .add_systems(
                OnEnter(GameState::Game),
                (
                    setup_spawn_config,
                    (spawn_terrain_for_selection, spawn_teams, map_battle_setup).after(setup_spawn_config),
                ),
            )
            .add_systems(Update, (update_camera, countdown).run_if(in_state(GameState::Game)))
            .add_systems(FixedUpdate, despawn_on_zero_health);

        self.finish(app);
    }

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        info!("Finish::MapBattlePlugin");
    }

    fn cleanup(&self, _app: &mut App) {
        info!("Cleanup::MapBattlePlugin");
    }

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// Builds and inserts [`SpawnConfig`] from the active [`MapSelection`].
///
/// Must run before [`spawn_teams`] in `OnEnter(GameState::Game)`.
fn setup_spawn_config(
    mut commands: Commands,
    selection: Res<MapSelection>,
    terrain: Res<TerrainConfig>,
) {
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
    info_once!("Setting up Logs of War game state");

    commands.spawn((
        DespawnOnExit(GameState::Game),
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.insert_resource(GlobalAmbientLight { brightness: 160.0, ..default() });

    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    mut game_state_writer: MessageWriter<GameStateEvent>,
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state_writer.write(GameStateEvent::GameComplete);
    }
}

/// Positions the camera to view the entire battlefield from an elevated angle.
fn update_camera(mut q: Query<&mut Transform, (With<Camera3d>, With<GameCamera>)>) {
    if let Ok(mut transform) = q.single_mut() {
        transform.translation = Vec3::new(0.0, 20.0, 25.0);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
