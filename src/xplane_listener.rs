use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, TaskPool};
use futures_lite::future;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;

const DEFAULT_ADDRESS: &str = "0.0.0.0:49000";

pub struct XPlaneListener;

impl Plugin for XPlaneListener {
    fn build(&self, app: &mut App) {
        AsyncComputeTaskPool::init(|| TaskPool::new());
        app.insert_resource(Network {
            socket: UdpSocket::bind(DEFAULT_ADDRESS)
                .expect(format!("Failed to bind address {}", DEFAULT_ADDRESS).as_str()),
        })
        .insert_resource(AircraftState { ..default() })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                trigger_task_system,
                resolve_task_system,
                tick_aircraft_time_system,
            ),
        );
    }
}

pub struct Payload {
    data: HashMap<i32, HashMap<i32, f32>>,
}

impl Payload {
    pub fn new() -> Self {
        let mut data: HashMap<i32, HashMap<i32, f32>> = HashMap::new();
        for x in 0..140 {
            data.insert(x, HashMap::new());
            for y in 0..8 {
                if let Some(record) = data.get_mut(&x) {
                    record.insert(y, 0.0);
                }
            }
        }
        Self { data }
    }

    pub fn insert(&mut self, x: i32, y: i32, value: f32) {
        if let Some(record) = self.data.get_mut(&x) {
            record.insert(y, value);
        }
    }

    pub fn loc(&mut self, x: i32, y: i32) -> Option<&f32> {
        if let Some(record) = self.data.get(&x) {
            return record.get(&y);
        }
        None
    }
}

#[derive(Resource)]
pub struct Network {
    socket: UdpSocket,
}

#[derive(Resource, Default, Debug)]
pub struct AircraftState {
    pub time: f32,
    pub pitch: f32,
    pub roll: f32,
    pub magnetic_heading: f32,
    pub true_heading: f32,
    pub indicated_airspeed: f32,
    pub equivalent_airspeed: f32,
    pub true_airspeed: f32,
    pub true_groundspeed: f32,
    pub mach: f32,
    pub vertical_speed: f32,
    pub latitude: f32,
    pub longitude: f32,
    pub mean_sea_level_altitude: f32,
    pub above_ground_level_altitude: f32,
    pub on_runway: f32,
    pub indicated_altitude: f32,
    pub origin_latitude: f32,
    pub origin_longitude: f32,
}

impl AircraftState {
    pub fn push(&mut self, mut payload: Payload) {
        if let Some(timestamp) = payload.loc(1, 0) {
            self.time = *timestamp;
            if let Some(value) = payload.loc(17, 0) {
                self.pitch = *value;
            }
            if let Some(value) = payload.loc(17, 1) {
                self.roll = *value;
            }
            if let Some(value) = payload.loc(17, 2) {
                self.magnetic_heading = *value;
            }
            if let Some(value) = payload.loc(17, 3) {
                self.true_heading = *value;
            }

            if let Some(value) = payload.loc(3, 0) {
                self.indicated_airspeed = *value;
            }
            if let Some(value) = payload.loc(3, 1) {
                self.equivalent_airspeed = *value;
            }
            if let Some(value) = payload.loc(3, 2) {
                self.true_airspeed = *value;
            }
            if let Some(value) = payload.loc(3, 3) {
                self.true_groundspeed = *value;
            }

            if let Some(value) = payload.loc(4, 0) {
                self.mach = *value;
            }
            if let Some(value) = payload.loc(4, 2) {
                self.vertical_speed = *value;
            }

            if let Some(value) = payload.loc(20, 0) {
                self.latitude = *value;
            }
            if let Some(value) = payload.loc(20, 1) {
                self.longitude = *value;
            }
            if let Some(value) = payload.loc(20, 2) {
                self.mean_sea_level_altitude = *value;
            }
            if let Some(value) = payload.loc(20, 3) {
                self.above_ground_level_altitude = *value;
            }
            if let Some(value) = payload.loc(20, 4) {
                self.on_runway = *value;
            }
            if let Some(value) = payload.loc(20, 5) {
                self.indicated_altitude = *value;
            }
            if let Some(value) = payload.loc(20, 6) {
                self.origin_latitude = *value;
            }
            if let Some(value) = payload.loc(20, 7) {
                self.origin_longitude = *value;
            }
        };
    }
}

#[derive(Component)]
pub struct TaskSchedule;

#[derive(Component)]
pub struct TaskResult(Task<Result<Payload, ()>>);

fn setup(mut commands: Commands) {
    commands.spawn(TaskSchedule {});
}

pub fn spawn_task(commands: &mut Commands, target: Entity, socket: UdpSocket) {
    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async move {
        let mut buf = [0; 1024];
        let (count, _source) = socket
            .recv_from(&mut buf)
            .expect("Error when receiving data!");
        let received_data = &buf[..count];
        process_udp_data(&received_data, &count)
    });
    commands.entity(target).insert(TaskResult(task));
}

pub fn trigger_task_system(
    mut commands: Commands,
    task_schedule_queryset: Query<Entity, (With<TaskSchedule>, Without<TaskResult>)>,
    network: Res<Network>,
) {
    for entity in &task_schedule_queryset {
        if let Ok(socket) = network.socket.try_clone() {
            spawn_task(&mut commands, entity, socket);
        }
    }
}

pub fn resolve_task_system(
    mut commands: Commands,
    mut task_queryset: Query<(Entity, &mut TaskResult)>,
    mut aircraft_state: ResMut<AircraftState>,
) {
    for (entity, mut task) in &mut task_queryset {
        if let Some(payload_result) = future::block_on(future::poll_once(&mut task.0)) {
            if let Ok(payload) = payload_result {
                aircraft_state.push(payload);
                commands.entity(entity).remove::<TaskResult>();
            }
        };
    }
}

fn tick_aircraft_time_system(mut aircraft_state: ResMut<AircraftState>, time: Res<Time>) {
    aircraft_state.time += time.delta_seconds();
}

fn process_udp_data(data: &[u8], count: &usize) -> Result<Payload, ()> {
    if &data[..4] == b"DATA" {
        let mut payload: Payload = Payload::new();
        // Check if the data length is not a multiple of 36 (invalid data)
        let raw = &data[(4 + 1)..*count];
        if raw.len() % 36 != 0 {
            return Err(());
        }

        // Calculate the number of records in the received data
        let num_records = raw.len() / 36;

        // Iterate through each record and process it
        for x in 0..num_records {
            let record_data = &raw[x * 36..(x + 1) * 36];
            // Extract the record number (first 4 bytes) and convert it to an integer
            let record_index = i32::from_le_bytes(
                record_data[..4]
                    .try_into()
                    .expect("Failed to read bytes while gathering record number!"),
            );
            // Extract the record values (remaining 32 bytes)
            let record_values = &record_data[4..];

            // Iterate through the 8 values within the record
            for y in 0..8 {
                let offset = y * 4; // Offset to the start of each value (4 bytes each)
                let value = f32::from_le_bytes(
                    record_values[offset..offset + 4]
                        .try_into()
                        .expect("Failed to read bytes when gathering record value!"),
                );
                payload.insert(record_index, y as i32, value);
            }
        }
        return Ok(payload);
    }
    Err(())
}
