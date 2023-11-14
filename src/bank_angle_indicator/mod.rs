use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct BankAngleIndicatorPlugin;

impl Plugin for BankAngleIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (systems::spawn_bank_angle_indicator,))
            .add_systems(Update, (systems::update_bank_angle_lines,));
    }
}
