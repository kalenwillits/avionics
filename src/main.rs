mod device;
mod displays;

use bevy::prelude::*;
use device::Device;
use displays::PrimaryFlightDisplay;




fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Device, PrimaryFlightDisplay))
        .run();
}

