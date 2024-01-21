use bevy::prelude::*;

pub mod resources;
pub mod settings;
pub mod utils;


pub struct ProfilesPlugin;

impl Plugin for ProfilesPlugin {
    fn build(&self, app: &mut App) {
        let profile = utils::load_profile();
        app.insert_resource(profile);
    }
}
