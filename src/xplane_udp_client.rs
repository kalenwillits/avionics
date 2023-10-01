use bevy::prelude::*;
use std::net::UdpSocket;
use std::convert::TryInto;

pub struct XPlaneUdpClient;

impl Plugin for XPlaneUdpClient {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AircraftState {
                pitch: 0.0, 
                roll: 0.0,
                magnetic_heading: 0.0,
                true_heading: 0.0,
            })
            .insert_resource( ClientConfig {
                socket: UdpSocket::bind("0.0.0.0:49000").unwrap()
            })
            .add_systems(Startup, setup)
            .add_systems(Update, poll_aircraft_state);
            // .add_systems(Update, print_aircraft_state);
    }
}


#[derive(Component)]
pub struct TickRate {
    timer: Timer,
}


#[derive(Resource, Debug)]
pub struct AircraftState {
    pub pitch: f32,
    pub roll: f32,
    pub magnetic_heading: f32,
    pub true_heading: f32,
}

#[derive(Resource)]
pub struct ClientConfig {
    socket: UdpSocket,
} 


fn setup(mut commands: Commands) {
    commands.spawn(TickRate {timer: Timer::from_seconds(0.05, TimerMode::Repeating)});
}





fn poll_aircraft_state(
    client_config: Res<ClientConfig>, 
    mut aircraft_state: ResMut<AircraftState>,
    time: Res<Time>,
    mut tick_rate_queryset: Query<&mut TickRate>,
    ) {
    for mut tick_rate in &mut tick_rate_queryset {
        tick_rate.timer.tick(time.delta());
        if !tick_rate.timer.finished() {
            return;
        }
        let mut buffer = [0; 1024];
        let (amount, _src) = client_config.socket.recv_from(&mut buffer).unwrap();
        let data = &buffer[..amount];
            if &data[..4] == b"DATA" {
               // process_udp_data(aircraft_state, data); 
                   // Check if the data length is not a multiple of 36 (invalid data)
    if data[5..amount].len() % 36 != 0 {
        println!("Invalid data length");
        return;
    }

    // Calculate the number of records in the received data
    let num_records = data[5..amount].len() / 36;


    // println!("Received {} records", num_records);

    // Iterate through each record and process it
    for i in 0..num_records {
        let record_data = &data[5..amount][i * 36..(i + 1) * 36];
        // process_record(aircraft_state, record_data);
                // Extract the record number (first 4 bytes) and convert it to an integer
        let record_index = i32::from_le_bytes(record_data[..4].try_into().unwrap());

        // Extract the record values (remaining 32 bytes)
        let record_values = &record_data[4..];

        // Print the record number
        // println!("Record #{}", record_index);

        // Iterate through the 8 values within the record
        for i in 0..8 {
            let offset = i * 4; // Offset to the start of each value (4 bytes each)

            // Extract the value as a 32-bit floating-point number and print it
            let value = f32::from_le_bytes(record_values[offset..offset + 4].try_into().unwrap());
            // println!("Value {}: {:.4}", i + 1, value);
            // match_value_to_aircraft_state(&aircraft_state, record_index, i, value);

            match record_index {
                17 => {
                    match i {
                        0 => {aircraft_state.pitch = value},
                        1 => {aircraft_state.roll = value},
                        2 => {aircraft_state.magnetic_heading = value},
                        3 => {aircraft_state.true_heading = value},
                        _ => (),
                    }

                },
                _ => (),

        };

        }


    }

            }
}
}

// fn print_aircraft_state(aircraft_state: Res<AircraftState>) {
//     println!("{:?}", aircraft_state);
// }
