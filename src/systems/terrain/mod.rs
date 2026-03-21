//! Terrain generation and spawning systems.
//!
//! Each submodule corresponds to one [`crate::resources::map_selection::MapSelection`] variant:
//! - [`hills`] — procedural sine-wave heightmap (Hills map)
//! - [`flat`] — flat cuboid (Testing Area, debug builds only)

pub mod hills;

#[cfg(debug_assertions)]
pub mod flat;

pub use hills::spawn_terrain;

#[cfg(debug_assertions)]
pub use flat::spawn_terrain_flat;
