use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default, Deserialize, Serialize)]
pub struct Profile {
    pub num_engines: i32,
}
