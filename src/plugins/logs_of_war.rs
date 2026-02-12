use avian3d::prelude::{AngularVelocity, Collider, RigidBody};
use bevy::{
    app::Update,
    ecs::{
        entity::EntityIndex, resource::Resource, schedule::IntoScheduleConfigs, system::{Commands, Res, ResMut, command}
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
    resources::game_state::{GameState, LogsOfWarGameState},
};

pub struct LogsOfWarPlugin;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

// #[derive(Resource)]
// struct GameWorld {
//     map: Entity,
//     cube: Entity,
// }


impl Plugin for LogsOfWarPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LogsOfWarGameState>()
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
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(400.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(4.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),

    ));

    // light
    commands.spawn((
        DespawnOnExit(GameState::Game),
        PointLight { shadows_enabled: true, ..default() },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));


    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn spawn_cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>) -> Entity {
    // Dynamic physics object with a collision shape and initial angular velocity
    let ent = commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 15.0)),
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 4.0, 0.0),
    )).id();

    ent
}

fn despawn_cube(mut commands: Commands, >) {
    commands.entity(entity)
}

// https://github.com/bevyengine/bevy/discussions/2658

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    mut inner_game_state: ResMut<NextState<LogsOfWarGameState>>,
    mut timer: ResMut<GameTimer>,
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
    time: Res<Time>,
) {
    if timer.tick(time.delta()).is_finished() {
        despawn_cube();
        inner_game_state.set(LogsOfWarGameState::Stopped);
        game_state.set(GameState::Menu);
    }
}

fn update_camera(mut q: Query<(&Camera3d, &mut Transform), With<GameCamera>>) {
    if let Ok(mut transform) = q.single_mut() {
        transform.1.translation = Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y).translation;
    }
}
