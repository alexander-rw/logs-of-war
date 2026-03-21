use bevy::prelude::*;

/// The map selected by the player before a battle.
///
/// Each variant corresponds to a terrain generator in `src/systems/terrain/`.
/// Adding a new variant requires a matching arm in `spawn_terrain_for_selection`
/// (in `map_battle.rs`) — omitting it causes a compile error.
#[derive(Clone, Copy, PartialEq, Debug, Default, Resource)]
pub enum MapSelection {
    #[default]
    Hills,

    #[cfg(debug_assertions)]
    TestingArea,
}

impl MapSelection {
    /// Returns a human-readable display name for use in the UI dropdown.
    ///
    /// Prefer this over `format!("{:?}", variant)`, which would give
    /// `"TestingArea"` instead of `"Testing Area"`.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Hills => "Hills",
            #[cfg(debug_assertions)]
            Self::TestingArea => "Testing Area",
        }
    }

    /// Returns all variants available in the current build configuration.
    ///
    /// Debug builds include [`MapSelection::TestingArea`]; release builds do not.
    ///
    /// This is the Rust equivalent of C#'s `Enum.GetValues<T>()`. The slice
    /// is a compile-time constant — no heap allocation occurs.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for &variant in MapSelection::all_variants() {
    ///     println!("{}", variant.label());
    /// }
    /// ```
    pub fn all_variants() -> &'static [Self] {
        // `#[cfg(...)]` on array elements is valid Rust — the compiler includes
        // or excludes that element at compile time, like C#'s `#if DEBUG`.
        #[cfg(debug_assertions)]
        return &[Self::Hills, Self::TestingArea];
        #[cfg(not(debug_assertions))]
        return &[Self::Hills];
    }
}
