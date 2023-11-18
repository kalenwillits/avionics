mod airspeed_indicator;
mod altimeter;
mod artificial_horizon;
mod bank_angle_indicator;
mod heading_indicator;
mod turn_coordinator;
mod vertical_speed_indicator;
mod panel_left;
mod panel_right;
mod tachometer;

mod xplane_listener;
mod utils;

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
            bank_angle_indicator::BankAngleIndicatorPlugin,
            airspeed_indicator::AirSpeedIndicatorPlugin,
            vertical_speed_indicator::VerticalSpeedIndicatorPlugin,
            turn_coordinator::TurnCoordinatorPlugin,
            altimeter::AltimeterPlugin,
            heading_indicator::HeadingIndicatorPlugin,
            panel_left::PanelLeftPlugin,
            panel_right::PanelRightPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
