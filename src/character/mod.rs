pub mod controller;

use bevy::prelude::*;
use bevy_tnua::TnuaSensorsSet;
use rand::seq::IndexedRandom;

use crate::character::controller::controller_plugin;
use crate::game::GameState;

/// Registers character control and the health rules.
pub fn character_plugin(app: &mut App) {
    app.add_plugins(controller_plugin)
        .add_observer(despawn_sensors_with_soldier)
        .add_systems(FixedUpdate, despawn_on_zero_health.run_if(in_state(GameState::Battle)));
}

/// Health every soldier starts a battle with.
const MAX_HEALTH: u16 = 100;

/// One log fighter in a battle.
#[derive(Component)]
#[require(Health = Health(MAX_HEALTH))]
pub struct Soldier;

/// How much damage a soldier can still take. A soldier on zero leaves the battle.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health(pub u16);

impl Health {
    /// Removes `damage` health, stopping at zero.
    // Nothing damages a soldier yet, so only the tests call this.
    #[allow(dead_code)]
    pub fn take_damage(&mut self, damage: u16) {
        self.0 = self.0.saturating_sub(damage);
    }

    /// Returns true while the soldier still has health.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.0 > 0
    }
}

/// The personal names a soldier can be given.
const NAMES: &[&str] = &[
    "Alice",
    "Bob",
    "Charlie",
    "Diana",
    "Eve",
    "Frank",
    "Grace",
    "Heidi",
    "Ivan",
    "Judy",
    "Abraham",
    "Ade",
    "Andy",
    "Badders",
    "Basil",
    "Bastille",
    "Ben",
    "Bobby-Jim",
    "Bobby-Joe",
    "Chucky",
    "Den",
    "Dolly",
    "Duski",
    "Fil",
    "Gerard",
    "Ginger",
    "Glouton",
    "Goinfre",
    "Herman",
    "Herr Dry",
    "Herr Gel",
    "Herr Kut",
    "Huski",
    "Izzy",
    "Jake",
    "James",
    "Jetski",
    "Jim",
    "Jim-Bob",
    "Joey-Bob",
    "John",
    "John-Boy",
    "Jones",
    "Keanu",
    "Lederhos",
    "Mark",
    "Martyn",
    "Monty",
    "Mule",
    "Muski",
    "Nobby",
    "Paul",
    "Percy",
    "Pesski",
    "Philip",
    "Ponsonby",
    "Porc",
    "Schnitzel",
    "Schwein",
    "Shogun",
    "Shorty",
    "Simon",
    "Sly",
    "Smith",
    "Sushi",
    "Sweety",
];

/// Returns one personal name, picked at random.
pub fn random_name() -> &'static str {
    NAMES.choose(&mut rand::rng()).expect("NAMES is never empty")
}

/// Despawns any soldier whose health has just reached zero.
fn despawn_on_zero_health(mut commands: Commands, query: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, health) in query.iter() {
        if !health.is_alive() {
            commands.entity(entity).despawn();
        }
    }
}

/// Despawns a soldier's Tnua ground sensor along with the soldier.
///
/// Tnua holds the sensor through the `TnuaSensorOf` relationship, not as a
/// child, and that relationship is not `linked_spawn`, so Bevy leaves the
/// sensor entity behind when the soldier despawns.
fn despawn_sensors_with_soldier(removed: On<Remove, Soldier>, sensors: Query<&TnuaSensorsSet>, mut commands: Commands) {
    let Ok(sensor_set) = sensors.get(removed.entity) else {
        return;
    };
    for sensor in sensor_set.iter() {
        commands.entity(sensor).try_despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_soldier_is_alive_on_full_health() {
        assert!(Health(MAX_HEALTH).is_alive());
    }

    #[test]
    fn take_damage_removes_that_much_health() {
        let mut health = Health(100);
        health.take_damage(30);
        assert_eq!(health, Health(70));
    }

    #[test]
    fn take_damage_stops_at_zero() {
        let mut health = Health(20);
        health.take_damage(50);
        assert_eq!(health, Health(0));
    }

    #[test]
    fn a_soldier_on_zero_health_is_not_alive() {
        assert!(!Health(0).is_alive());
    }

    #[test]
    fn random_name_comes_from_the_name_list() {
        assert!(NAMES.contains(&random_name()));
    }

    #[test]
    fn the_tnua_sensor_goes_with_the_soldier() {
        let mut app = App::new();
        app.add_observer(despawn_sensors_with_soldier);

        let soldier = app.world_mut().spawn(Soldier).id();
        let sensor = app.world_mut().spawn(bevy_tnua::TnuaSensorOf(soldier)).id();

        app.world_mut().entity_mut(soldier).despawn();
        app.world_mut().flush();

        assert!(app.world().get_entity(sensor).is_err(), "the sensor entity outlived the soldier");
    }
}
