// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, bevy::prelude::States)]
pub(crate) enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}