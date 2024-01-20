use bevy::prelude::*;

pub mod systems;
pub mod resources;
pub mod settings;

pub struct ProfilesPlugin;


impl Plugin for ProfilesPlugin {
    fn build(&self, app: &mut App) {
       app.insert_resource(resources::Profile { ..default() })
           .add_systems(Startup, systems::load_profile);
    }
}
