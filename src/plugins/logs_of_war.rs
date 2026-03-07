use avian3d::prelude::*;

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
    systems::despawn_on_should_despawn_true::despawn_character,
    systems::terrain::spawn_terrain,
};

pub struct LogsOfWarPlugin;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for LogsOfWarPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LogsOfWarGameState>()
            .add_systems(OnEnter(GameState::Game), (spawn_terrain, game_setup))
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

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info_once!("Setting up Logs of War game state");

    // light
    commands.spawn((
        DespawnOnExit(GameState::Game),
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    spawn_cube(&mut commands, &mut meshes, &mut materials);

    let size: Vec3 = Vec3::new(0.5, 5.0, 0.5);

    let player_test = (
        RigidBody::Dynamic,
        Collider::cuboid(size.x, size.y, size.z),
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 125))),
        LogCharacter::generate(),
    );

    commands.spawn(player_test);

    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn spawn_cube(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> Entity {
    // Dynamic physics object with a collision shape and initial angular velocity
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            AngularVelocity(Vec3::new(2.5, 3.5, 15.0)),
            Mesh3d(meshes.add(Cuboid::from_length(1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 4.0, 0.0),
            LogCharacter::generate(),
        ))
        .id()
}
// https://github.com/bevyengine/bevy/discussions/2658
// Tick the timer, and change state when finished

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    mut inner_game_state: ResMut<NextState<LogsOfWarGameState>>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<(Entity, &mut LogCharacter)>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).is_finished() {
        for (entity, mut log_char) in query.iter_mut() {
            info!("Found e: {0}, {1}", entity.index_u32(), log_char.name);
            log_char.take_damage(100);
            info!("Health: {0}, {1}", log_char.name, log_char.health);
        }
        inner_game_state.set(LogsOfWarGameState::Stopped);
        game_state.set(GameState::Menu);
    }
}

fn update_camera(mut q: Query<(&Camera3d, &mut Transform), With<GameCamera>>) {
    if let Ok(mut transform) = q.single_mut() {
        transform.1.translation = Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y).translation;
    }
}
