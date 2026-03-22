use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use avian3d::prelude::*;

pub struct PhysicsBasePlugin;

impl Plugin for PhysicsBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_paused.run_if(input_just_pressed(KeyCode::Escape)),
                step.run_if(physics_paused.and(input_just_pressed(KeyCode::Enter))),
            ),
        );
            // .add_systems(FixedUpdate, ());

        self.finish(app);
    }

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        info!("{0}::Finish", self.name());
    }

    fn cleanup(&self, _app: &mut App) {
        info!("{0}::Cleanup", self.name());
    }

    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn physics_paused(time: Res<Time<Physics>>) -> bool {
    time.is_paused()
}

fn toggle_paused(mut time: ResMut<Time<Physics>>) {
    info_once!("toggle_paused ran");
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
