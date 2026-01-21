use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::prelude::{App, Plugin, Timer, TimerMode, Startup, Update};

use crate::queries::people_query::{add_people, greet_people, update_people};
use crate::resources::greet_timer::GreetTimer;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}