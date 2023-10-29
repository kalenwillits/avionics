use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct HeadingIndicatorPlugin;

impl Plugin for HeadingIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_heading_indicator)
        .add_systems(Update, systems::update_heading_indicator);
    }
}
