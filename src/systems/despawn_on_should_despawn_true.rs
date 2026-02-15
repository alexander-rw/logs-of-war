use bevy::ecs::{entity::Entity, system::{Commands, Query}};

use crate::{log_character::log_character::LogCharacter, plugins::logs_of_war::ShouldDespawn};

/// Despawn all entities marked with `ShouldDespawn(true)`.
///
/// Iterates over every entity that has a `ShouldDespawn` component and
/// despawns those whose inner flag is `true`.
///
/// # Arguments
///
/// * `commands` - Bevy command queue used to schedule entity despawns
/// * `query_should_despawn` - Query over entities that carry a `ShouldDespawn` component
pub fn despawn_on_should_despawn_true(
    mut commands: Commands,
    query_should_despawn: Query<(Entity, &LogCharacter, &ShouldDespawn)>,
) {
    // .iter() returns shared references — matching the & in &ShouldDespawn.
    // In Rust, mutability must be declared at every level: the query type,
    // the iterator method, and the reference in the destructure must all agree.
    for (entity, log_char, should_despawn) in query_should_despawn.iter() {
        // should_despawn is &ShouldDespawn, a plain struct reference —
        // not a Result<T, E>, so we access the inner bool with .0
        // (tuple-struct field access, like positional indexing in C# records).
        // 0 here is the position of the bool field in the ShouldDespawn struct, which is a tuple struct, 
        // so it uses .0 to access the first field. If ShouldDespawn had named fields, we would use .field_name instead (like other languages).
        if should_despawn.0 || !log_char.is_alive() {
            commands.entity(entity).try_despawn();
        }
    }
}
