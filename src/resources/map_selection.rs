use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default, Resource)]
pub enum MapSelection {
    #[default]
    Hills,
}

