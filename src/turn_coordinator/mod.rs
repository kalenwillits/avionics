use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct TurnCoordinatorPlugin;

impl Plugin for TurnCoordinatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_turn_coordinator)
            .add_systems(Update, systems::update_turn_coordinator);
    }
}
