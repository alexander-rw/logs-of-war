use bevy::{
    app::Update, ecs::{resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, Res, ResMut}}, prelude::*, state::{app::AppExtStates, condition::in_state, state::{NextState, OnEnter}}, time::{Time, Timer, TimerMode}
};

use crate::{components::game_camera::GameCamera, resources::game_state::{GameState, LogsOfWarGameState}};

pub struct LogsOfWarPlugin;


#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

impl Plugin for LogsOfWarPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<LogsOfWarGameState>()
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, (update_camera, countdown).run_if(in_state(GameState::Game)));

        self.ready(app);
    }
    
    fn ready(&self, _app: &App) -> bool {
        true
    }
    
    fn finish(&self, _app: &mut App) {
        print!("Finish::LogsOfWarPlugin");
    }
    
    fn cleanup(&self, _app: &mut App) {
        print!("Cleanup::LogsOfWarPlugin");
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
    println!("Setting up Logs of War game state");
    // Spawn a 5 seconds timer to trigger going back to the menu
    // circular base
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        DespawnOnExit(GameState::Game), PointLight { shadows_enabled: true, ..default() }, Transform::from_xyz(4.0, 8.0, 4.0)));
    
    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(mut game_state: ResMut<NextState<GameState>>, mut inner_game_state: ResMut<NextState<LogsOfWarGameState>>, mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if timer.tick(time.delta()).is_finished() {
        inner_game_state.set(LogsOfWarGameState::Stopped);
        game_state.set(GameState::Menu);
    }
}

// fn despawn_on_exit_component<T: Bundle>(b: T) -> EntityCommands<'static> {
//     (
//         DespawnOnExit(GameState::Game),
//         b,
//     )
// }


fn update_camera(mut q: Query<(&Camera3d, &mut Transform), With<GameCamera>>) {
    if let Ok(mut transform) = q.single_mut() {
        transform.1.translation = Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y).translation;
    }
    // transform.translation = Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y).translation;
}