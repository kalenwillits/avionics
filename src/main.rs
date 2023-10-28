mod utils;
mod xplane_listener;
mod airspeed_indicator;
mod artificial_horizon;

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
            WorldInspectorPlugin::default(),
            xplane_listener::XPlaneListener,
            artificial_horizon::ArtificialHorizonPlugin,
            airspeed_indicator::AirSpeedIndicatorPlugin,
        ))
        .run();
}
