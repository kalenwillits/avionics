use bevy::prelude::*;

use super::resources;
use crate::profiles::settings;

use std::path::Path;
use toml;

fn create_profile(profile_filename: &str, profile: resources::Profile) {
    let _ = std::fs::write(profile_filename, toml::to_string(&profile).expect("Unable to write to file"));
}

pub fn load_profile() -> resources::Profile {
    let profile_filename: String = match std::env::var(settings::AIRCRAFT_PROFILE_ENVVAR) {
        Ok(filename) => filename,
        Err(_err) => {
            std::env::set_var(settings::AIRCRAFT_PROFILE_ENVVAR, settings::AIRCRAFT_PROFILE_FILENAME);
            settings::AIRCRAFT_PROFILE_FILENAME.to_string()
        }
    };

    let aircraft_profile_path: &Path = Path::new(&profile_filename);

    if !(aircraft_profile_path.exists()) {
         create_profile(
            settings::AIRCRAFT_PROFILE_FILENAME,
            resources::Profile { ..default() },
        );

    }
    let profile_string: String = std::fs::read_to_string(aircraft_profile_path).expect("Unable to read profile");
    let profile: resources::Profile = toml::from_str::<resources::Profile>(profile_string.as_str()).expect("Invalid Profile");
    profile
}
