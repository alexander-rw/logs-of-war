use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
    log::warn_once,
};

use crate::log_character::log_character::LogCharacter;

pub fn despawn_character(mut commands: Commands, query: Query<(Entity, &LogCharacter)>) {
    for (entity, log_char) in query.iter() {
        warn_once!("Char id {0}, health: {1}", entity.index(), &log_char.health);
        if !log_char.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}
