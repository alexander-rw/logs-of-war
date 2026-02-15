use bevy::ecs::component::Component;
// Trait import required to call .random_range() on the RNG instance.
// In Rust, trait methods are only available when the trait is in scope —
// similar to how C# extension methods need a `using` for their namespace.
use rand::Rng;

#[derive(Component)]
pub struct LogCharacter {
    pub name: String,
    pub health: u32,
}

fn random_name() -> String {
    let names = vec![
        "Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Heidi", "Ivan", "Judy",
    ];
    // random_range generates a value directly within [0, len), avoiding
    // modulo bias. Rust's rand crate doesn't support random::<usize>() in
    // v0.9 — you must use the Rng trait methods on a generator instance.
    let index = rand::rng().random_range(0..names.len());
    names[index].to_string()
}

impl LogCharacter {
    pub fn generate() -> Self {
        LogCharacter {
            name: random_name(),
            health: 100,
        }
    }

    pub fn new(name: String, health: u32) -> Self {
        LogCharacter { name, health }
    }

    pub fn take_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = std::cmp::max(0,  self.health - damage);
        } else {
            self.health -= damage;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}