pub mod teams;

use bevy::prelude::*;

use crate::character::despawn_on_zero_health;
use crate::game::GameState;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

pub fn battle_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Battle), map_battle_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Battle)))
        .add_systems(FixedUpdate, despawn_on_zero_health);
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

fn countdown(mut game_state: ResMut<NextState<GameState>>, mut timer: ResMut<GameTimer>, time: Res<Time>) {
    if timer.tick(time.delta()).just_finished() {
        game_state.set(GameState::Menu);
    }
}
