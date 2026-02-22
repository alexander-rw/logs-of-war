use bevy::{asset::AssetServer, ecs::component::Component, log::info_once};
// Trait import required to call .random_range() on the RNG instance.
// In Rust, trait methods are only available when the trait is in scope —
// similar to how C# extension methods need a `using` for their namespace.
use rand::Rng;

#[derive(Component)]
pub struct LogCharacter {
    pub name: String,
    pub health: i16,
}

fn random_name() -> String {
    let names = vec![
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
        "Le Cont",
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

    // random_range generates a value directly within [0, len), avoiding
    // modulo bias. Rust's rand crate doesn't support random::<usize>() in
    // v0.9 — you must use the Rng trait methods on a generator instance.
    let index = rand::rng().random_range(0..names.len());
    names[index].to_string()
}

impl LogCharacter {
    pub fn generate() -> Self {
        LogCharacter { name: random_name(), health: 100 }
    }

    pub fn take_damage(&mut self, damage: i16) {
        let health_diff = self.health - damage;
        self.health = std::cmp::max(0, health_diff);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
