use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct TachometerPlugin;

impl Plugin for TachometerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_tachometer)
            .add_systems(Update, systems::update_tachometer);
    }
}
