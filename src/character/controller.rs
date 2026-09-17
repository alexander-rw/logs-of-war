//! Character control scheme and player marker for Tnua-driven movement.

use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::prelude::*;

/// Distance the character origin floats above the ground.
///
/// Roughly the capsule half-extent (`half_height + radius`) plus a small hover
/// gap, so the body rests just above the surface.
pub const FLOAT_HEIGHT: f32 = 1.0;

/// Peak height of a jump, in world units.
pub const JUMP_HEIGHT: f32 = 3.0;

/// Horizontal walk speed, in world units per second.
pub const WALK_SPEED: f32 = 12.0;

/// Tnua control scheme: a walking basis plus a single jump action.
///
/// Deriving [`TnuaScheme`] generates the matching `ControlSchemeConfig` asset
/// (with `basis` and `jump` fields) used to tune the controller.
#[derive(TnuaScheme)]
#[scheme(basis = TnuaBuiltinWalk)]
pub enum ControlScheme {
    Jump(TnuaBuiltinJump),
}

/// Marks the single character that responds to keyboard input.
#[derive(Component)]
pub struct PlayerControlled;

/// Registers the Tnua controller and Avian backend, and drives the
/// player-controlled character from the keyboard.
///
/// Both Tnua plugins run in `FixedUpdate` to match Avian's fixed-timestep
/// simulation.
pub fn character_plugin(app: &mut App) {
    app.add_plugins((TnuaControllerPlugin::<ControlScheme>::new(FixedUpdate), TnuaAvian3dPlugin::new(FixedUpdate)))
        .add_systems(Update, apply_controls.in_set(TnuaUserControlsSystems));
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
