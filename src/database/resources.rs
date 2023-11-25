use bevy::prelude::*;
use sqlite;

#[derive(Resource)]
pub struct Database {
    pub connection: sqlite::ConnectionThreadSafe,
}
