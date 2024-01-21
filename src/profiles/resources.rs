use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Deserialize, Serialize)]
pub struct Profile {
    pub engine_one: Guage,
    pub engine_two: Guage,
    pub engine_three: Guage,
    pub engine_four: Guage,
}


#[derive(Default, Deserialize, Serialize)]
pub struct Guage {
    pub enabled: bool,
    pub range: Range,
    pub normal: Range,
    pub warning: Range,
    pub danger: Range,
}


#[derive(Default, Deserialize, Serialize)]
pub struct Range {
    pub min: i32, 
    pub max: i32,
}
