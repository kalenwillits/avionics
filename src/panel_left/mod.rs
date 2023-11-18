use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PanelLeftPlugin;

impl Plugin for PanelLeftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_panel_left);
    }
}

