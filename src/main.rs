mod device;
mod primary_flight_display;
mod utils;
mod xplane_listener;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const STEAM_DECK_RESOLUTION: (f32, f32) = (1280f32, 800f32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Avionics".into(),
                    resolution: STEAM_DECK_RESOLUTION.into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            // WorldInspectorPlugin::default(),
            device::Device,
            xplane_listener::XPlaneListener,
            primary_flight_display::PrimaryFlightDisplay,
        ))
        .run();
}
