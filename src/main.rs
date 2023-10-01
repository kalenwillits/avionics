mod device;
mod displays;
mod xplane_udp_client;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::prelude::*;


const STEAM_DECK_RESOLUTION: (f32, f32) = (1280f32, 800f32);


fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins.
                set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Avionics".into(),
                        resolution: STEAM_DECK_RESOLUTION.into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }), 
                WorldInspectorPlugin::default(),
                device::Device, 
                xplane_udp_client::XPlaneUdpClient,
                displays::PrimaryFlightDisplay,
            ))
        .run();
}

