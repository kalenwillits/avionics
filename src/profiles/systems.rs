use bevy::prelude::*;

use crate::profiles::components;
use crate::profiles::settings;

use std::path::Path;
use toml;

fn create_profile(profile_filename: &str, profile: components::Profile) {
    let _ = std::fs::write(profile_filename, toml::to_string(&profile).unwrap());
}

pub fn load_profile(mut commands: Commands) {
    let profile_filename: String = match std::env::var(settings::AIRCRAFT_PROFILE_ENVVAR) {
        Ok(filename) => filename,
        Err(_err) => {
            create_profile(
                settings::AIRCRAFT_PROFILE_FILENAME,
                components::Profile { ..default() },
            );
            std::env::set_var(settings::AIRCRAFT_PROFILE_ENVVAR, settings::AIRCRAFT_PROFILE_FILENAME);
            settings::AIRCRAFT_PROFILE_FILENAME.to_string()
        }
    };

    let aircraft_profile_path: &Path = Path::new(&profile_filename);
    let profile_string: String = std::fs::read_to_string(aircraft_profile_path).expect("Unable to read profile");
    let profile: components::Profile = toml::from_str::<components::Profile>(profile_string.as_str()).expect("Invalid Profile");
    commands.spawn(profile);
}
