use avian3d::prelude::*;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn physics_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            toggle_paused.run_if(input_just_pressed(KeyCode::Escape)),
            step.run_if(physics_paused.and(input_just_pressed(KeyCode::Enter))),
        ),
    );
}

fn physics_paused(time: Res<Time<Physics>>) -> bool {
    time.is_paused()
}

fn toggle_paused(mut time: ResMut<Time<Physics>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

/// Advances the physics simulation by one `Time<Fixed>` time step.
fn step(mut physics_time: ResMut<Time<Physics>>, fixed_time: Res<Time<Fixed>>) {
    physics_time.advance_by(fixed_time.delta());
}
