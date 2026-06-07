use bevy::prelude::*;
use bevy_tnua::builtins::TnuaBuiltinWalk;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::prelude::*;

use crate::components::controller::{ControlScheme, PlayerControlled, WALK_SPEED};

/// Registers the Tnua controller and Avian backend, and drives the
/// player-controlled character from the keyboard.
///
/// Both Tnua plugins run in `FixedUpdate` to match Avian's fixed-timestep
/// simulation.
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate), TnuaAvian3dPlugin::new(FixedUpdate)))
            .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystems));
    }
}

/// Translates WASD movement and Space jumps into Tnua controller commands for
/// the [`PlayerControlled`] character.
fn apply_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut TnuaController<ControlScheme>, With<PlayerControlled>>,
) {
    let Ok(mut controller) = query.single_mut() else {
        return;
    };
    controller.initiate_action_feeding();

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction -= Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction += Vec3::Z;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= Vec3::X;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += Vec3::X;
    }

    controller.basis = TnuaBuiltinWalk { desired_motion: direction.normalize_or_zero() * WALK_SPEED, ..default() };

    if keyboard.pressed(KeyCode::Space) {
        controller.action(ControlScheme::Jump(default()));
    }
}
