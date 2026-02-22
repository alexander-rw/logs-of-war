use bevy::{ecs::{
    entity::Entity,
    system::{Commands, Query},
}, log::info_once};

use crate::{log_character::log_character::LogCharacter, plugins::logs_of_war::ShouldDespawn};

pub fn despawn_character(
    mut commands: Commands,
    query: Query<(Entity, &LogCharacter)>,
) {
    // .iter() returns shared references — matching the & in &ShouldDespawn.
    // In Rust, mutability must be declared at every level: the query type,
    // the iterator method, and the reference in the destructure must all agree.
    for (entity, log_char) in query.iter() {
        // should_despawn is &ShouldDespawn, a plain struct reference —
        // not a Result<T, E>, so we access the inner bool with .0
        // (tuple-struct field access, like positional indexing in C# records).
        // 0 here is the position of the bool field in the ShouldDespawn struct, which is a tuple struct,
        // so it uses .0 to access the first field. If ShouldDespawn had named fields, we would use .field_name instead (like other languages).
        let char_is_alive = log_char.is_alive();
        info_once!("Char is alive: {0}, {1}", entity.index(), char_is_alive);
        if !char_is_alive {
            commands.entity(entity).despawn();
        }
    }
}
