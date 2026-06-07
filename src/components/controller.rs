//! Character control scheme and player marker for Tnua-driven movement.

use bevy::prelude::*;
use bevy_tnua::builtins::{TnuaBuiltinJump, TnuaBuiltinWalk};
use bevy_tnua::prelude::*;

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
