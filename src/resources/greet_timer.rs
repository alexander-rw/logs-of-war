use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub(crate) struct GreetTimer(pub(crate) Timer);