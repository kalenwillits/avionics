mod device;
mod displays;

use device::Device;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;
use displays::PrimaryFlightDisplay;




fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.
                set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Avionics".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }), 
                WorldInspectorPlugin::default(),
                Device, 
                PrimaryFlightDisplay,
            ))
        .run();
}

