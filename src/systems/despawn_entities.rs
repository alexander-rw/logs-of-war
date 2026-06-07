use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query},
    },
    log::warn_once,
};

use crate::components::character::TreeCharacter;

pub fn despawn_on_zero_health(mut commands: Commands, query: Query<(Entity, &TreeCharacter)>) {
    for (entity, character) in query.iter() {
        warn_once!("Char id {0} ({1}), health: {2}", entity.index(), &character.name, &character.health);
        if !character.is_alive() {
            commands.entity(entity).despawn_children();
            commands.entity(entity).despawn();
        }
    }
}
