use bevy::prelude::*;

pub mod components;
pub mod settings;
pub mod systems;

pub struct ProfilesPlugin;

impl Plugin for ProfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::load_profile);
    }
}
