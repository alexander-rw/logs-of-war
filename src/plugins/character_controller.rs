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

/// Walk direction contributed by each movement key. Bevy is Z-forward-negative,
/// so W subtracts Z and S adds it.
const MOVEMENT_KEYS: [(KeyCode, Vec3); 4] =
    [(KeyCode::KeyW, Vec3::NEG_Z), (KeyCode::KeyS, Vec3::Z), (KeyCode::KeyA, Vec3::NEG_X), (KeyCode::KeyD, Vec3::X)];

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

    let direction: Vec3 =
        MOVEMENT_KEYS.iter().filter(|(key, _)| keyboard.pressed(*key)).map(|(_, direction)| *direction).sum();

    controller.basis = TnuaBuiltinWalk { desired_motion: direction.normalize_or_zero() * WALK_SPEED, ..default() };

    if keyboard.pressed(KeyCode::Space) {
        controller.action(ControlScheme::Jump(default()));
    }
}
