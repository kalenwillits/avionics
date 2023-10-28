use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct ArtificialHorizonPlugin;

impl Plugin for ArtificialHorizonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                systems::spawn_artificial_horizon,
                systems::spawn_pitch_lines,
                systems::spawn_crosshairs,
            ),
        )
        .add_systems(
            Update,
            (
                systems::update_artificial_horizon,
                systems::update_pitch_lines,
            ),
        );
    }
}
