mod artificial_horizon;
mod airspeed_indicator;
mod altimeter;
mod utils;
mod xplane_listener;

use bevy::prelude::*;

const STEAM_DECK_RESOLUTION: (f32, f32) = (1280f32, 800f32);

fn main() {
    App::new()
        .add_systems(Startup, setup)
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
            xplane_listener::XPlaneListener,
            artificial_horizon::ArtificialHorizonPlugin,
            airspeed_indicator::AirSpeedIndicatorPlugin,
            altimeter::AltimeterPlugin,
        ))
        .run();
}

fn setup(
    mut commands: Commands
    ) {
    commands.spawn(Camera2dBundle::default());
}
