use bevy::{
    app::Update,
    ecs::{
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Res, ResMut},
    },
    prelude::*,
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{NextState, OnEnter},
    },
    time::{Time, Timer, TimerMode},
};

use crate::{
    components::game_camera::GameCamera,
    log_character::log_character::LogCharacter,
    resources::game_state::{GameState, LogsOfWarGameState},
    resources::terrain_config::TerrainConfig,
    systems::despawn_on_should_despawn_true::despawn_character,
    systems::spawn_teams::spawn_teams,
    systems::terrain::spawn_terrain,
};

pub struct LogsOfWarPlugin;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for LogsOfWarPlugin {
    fn build(&self, app: &mut App) {
        // Insert TerrainConfig resource with default values
        app.insert_resource(TerrainConfig::default());

        app.init_state::<LogsOfWarGameState>()
            .add_systems(OnEnter(GameState::Game), (spawn_terrain, spawn_teams, game_setup))
            .add_systems(Update, (update_camera, countdown).run_if(in_state(GameState::Game)))
            .add_systems(FixedUpdate, despawn_character);

        self.ready(app);
    }

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        info!("Finish::LogsOfWarPlugin");
    }

    fn cleanup(&self, _app: &mut App) {
        info!("Cleanup::LogsOfWarPlugin");
    }

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// Sets up game lighting and timer.
fn game_setup(mut commands: Commands) {
    info_once!("Setting up Logs of War game state");

    // Point light for scene illumination
    commands.spawn((
        DespawnOnExit(GameState::Game),
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Timer for demo purposes (will be replaced by turn system)
    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}
// https://github.com/bevyengine/bevy/discussions/2658
// Tick the timer, and change state when finished

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    mut inner_game_state: ResMut<NextState<LogsOfWarGameState>>,
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).is_finished() {
        inner_game_state.set(LogsOfWarGameState::Stopped);
        game_state.set(GameState::Menu);
    }
}

/// Positions the camera to view the entire battlefield from an elevated angle.
fn update_camera(mut q: Query<&mut Transform, (With<Camera3d>, With<GameCamera>)>) {
    if let Ok(mut transform) = q.single_mut() {
        // Elevated position looking down at center of terrain
        transform.translation = Vec3::new(0.0, 20.0, 25.0);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
