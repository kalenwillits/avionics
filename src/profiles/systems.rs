use bevy::prelude::*;

use crate::profiles::settings;
use crate::profiles::resources;

use std::path::Path;

pub fn load_profile(
        mut commands: Commands, 
        profile: Res<resources::Profile>) {
    let profile_filename: String = match std::env::var(settings::AIRCRAFT_PROFILE_ENVVAR) {
        Ok(filename) => filename,
        Err(_err) => {
            // TODO - create profile file
            settings::AIRCRAFT_PROFILE_FILENAME.to_string()
        },
    };

    let aircraft_profile_path: &Path = Path::new(&profile_filename);
    if !aircraft_profile_path.exists() {
        
    }
}

