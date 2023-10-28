use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct AltimeterPlugin;

impl Plugin for AltimeterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_altimeter)
            .add_systems(Update, systems::update_altimeter);
    }
}

