use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct PanelRightPlugin;

impl Plugin for PanelRightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_panel_right);
    }
}

