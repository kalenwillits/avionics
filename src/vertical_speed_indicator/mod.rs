use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct VerticalSpeedIndicatorPlugin;

impl Plugin for VerticalSpeedIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_vertical_speed_indicator)
            .add_systems(Update, systems::update_vertical_speed_indicator);
    }
}
