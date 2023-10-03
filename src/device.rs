use bevy::prelude::*;

pub struct Device;

impl Plugin for Device {
    fn build(&self, app: &mut App) {
        app.add_state::<DisplayState>().add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum DisplayState {
    #[default]
    PrimaryFlightDisplay,
    Off,
}
