use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
    log::warn_once,
};

use crate::entities::character::tree_character::TreeCharacter;

pub fn despawn_on_zero_health(mut commands: Commands, query: Query<(Entity, &TreeCharacter)>) {
    for (entity, log_char) in query.iter() {
        warn_once!("Char id {0}, health: {1}", entity.index(), &log_char.health);
        if !log_char.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}
