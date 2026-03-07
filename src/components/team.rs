//! Team identification components for log soldiers.
//!
//! This module provides the [`Team`] component and [`TeamId`] enum
//! for distinguishing between opposing teams in the game.

use bevy::color::Color;
use bevy::ecs::component::Component;

/// Unique identifier for each team.
///
/// In the MVP, there are exactly two teams: Red and Blue.
/// Each team has an associated display color and human-readable name.
///
/// # Examples
///
/// ```
/// use logs_of_war::components::team::TeamId;
///
/// let team = TeamId::Red;
/// assert_eq!(team.name(), "Red Team");
/// ```
// Note for Python developers: Rust enums are more powerful than Python enums.
// Each variant can hold data (not used here), and you can implement methods
// directly on the enum type via `impl` blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TeamId {
    /// The red team, spawning on the negative X side of the map.
    Red,
    /// The blue team, spawning on the positive X side of the map.
    Blue,
}

impl TeamId {
    /// Returns the team's display color for meshes and UI elements.
    ///
    /// # Returns
    ///
    /// A Bevy [`Color`] representing the team's primary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use logs_of_war::components::team::TeamId;
    ///
    /// let red_color = TeamId::Red.color();
    /// let blue_color = TeamId::Blue.color();
    /// ```
    // Note for Python developers: `&self` is similar to `self` in Python methods,
    // but the `&` means we're borrowing (not consuming) the value. This allows
    // the caller to continue using the TeamId after calling this method.
    #[must_use]
    pub fn color(&self) -> Color {
        match self {
            TeamId::Red => Color::srgb(0.85, 0.2, 0.2),  // Crimson red
            TeamId::Blue => Color::srgb(0.2, 0.4, 0.85), // Royal blue
        }
    }

    /// Returns a human-readable name for the team.
    ///
    /// # Returns
    ///
    /// A static string slice containing the team's display name.
    ///
    /// # Examples
    ///
    /// ```
    /// use logs_of_war::components::team::TeamId;
    ///
    /// assert_eq!(TeamId::Red.name(), "Red Team");
    /// assert_eq!(TeamId::Blue.name(), "Blue Team");
    /// ```
    // Note for Python developers: `&'static str` is a string slice with a
    // 'static lifetime, meaning it lives for the entire program duration.
    // This is efficient because we're returning references to string literals
    // embedded in the binary, avoiding heap allocation.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            TeamId::Red => "Red Team",
            TeamId::Blue => "Blue Team",
        }
    }
}

/// Component that identifies which team an entity belongs to.
///
/// Attach this component to log soldier entities to assign them to a team.
/// The component wraps a [`TeamId`] and provides access to team-specific
/// properties like color and name.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use logs_of_war::components::team::{Team, TeamId};
///
/// fn spawn_soldier(mut commands: Commands) {
///     commands.spawn(Team { id: TeamId::Red });
/// }
/// ```
// Note for Python developers: In Rust, we often wrap simple enums in a struct
// when using them as ECS components. This allows the struct to derive the
// Component trait while keeping the enum focused on its domain logic.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Team {
    /// The team identifier for this entity.
    pub id: TeamId,
}

