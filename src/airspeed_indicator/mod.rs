use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct AirSpeedIndicatorPlugin;

impl Plugin for AirSpeedIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_airspeed_indicator)
            .add_systems(Update, systems::update_airspeed_indicator);
    }
}
