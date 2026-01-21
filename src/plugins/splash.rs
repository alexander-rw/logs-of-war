use bevy::prelude::*;

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

use crate::resources::game_state::GameState;

// This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
pub fn splash_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("logs-of-war.png");
    // Display the logo
    commands.spawn((
        // This entity will be despawned when exiting the state
        DespawnOnExit(GameState::Splash),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        OnSplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: px(200),
                ..default()
            },
        )],
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(GameState::Menu);
    }
}