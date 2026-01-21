use bevy::Query;

use crate::components::name::Name;
use crate::components::person::Person;

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}