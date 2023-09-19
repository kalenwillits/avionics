use bevy::prelude::*;


pub struct Device;

impl Plugin for Device {
    fn build(&self, app: &mut App) {
        app
            .add_state::<DisplayState>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum DisplayState {
    #[default]
    Off,
    PrimaryFlightDisplay,
    MultiFunctionDisplay,
    GlobalPositioningSystem,
    Radio,
    Switches,
}

