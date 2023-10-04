use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, TaskPool};
use futures_lite::future;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;

const DEFAULT_ADDRESS: &str = "0.0.0.0:49000";

pub struct XPlaneListener;

// type Record = HashMap<i32, f32>;
// type Payload = HashMap<i32, Record>;

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

#[derive(Default, Debug, Clone, Copy)]
pub struct TimeKey {
    timestamp: f32,
    value: f32,
}

#[derive(Default, Debug)]
pub struct TimeFrame {
    pub key: Option<TimeKey>,
    pub next: Option<TimeKey>,
}

impl TimeFrame {
    pub fn interpolate(&self, time: f32) -> f32 {
        if let Some(key) = self.key {
            if let Some(next) = self.next {
                let latency: f32 = next.timestamp - key.timestamp;
                let timedelta: f32 = next.timestamp - time;
                let delta_factor: f32 = timedelta / latency;
                let value_delta: f32 = (next.value - key.value) * delta_factor;
                return key.value + value_delta;
            } else {
                // No next frame has been placed yet.
                return key.value;
            }
        } else {
            // There is not yet enough data to interpolate
            return 0.0;
        }
    }

    pub fn push(&mut self, value_option: Option<&f32>, timestamp: f32) {
        if let Some(value) = value_option {
            self.next = self.key;
            self.key = Some(TimeKey {
                timestamp: timestamp,
                value: *value,
            });
        };
    }
}

#[derive(Resource, Default, Debug)]
pub struct AircraftState {
    pub time: f32,
    pub pitch: TimeFrame,
    pub roll: TimeFrame,
    pub magnetic_heading: TimeFrame,
    pub true_heading: TimeFrame,
}

impl AircraftState {
    pub fn push(&mut self, mut payload: Payload) {
        if let Some(timestamp) = payload.loc(1, 0) {
            self.time = *timestamp;
            self.pitch.push(payload.loc(17, 0), self.time);
            self.roll.push(payload.loc(17, 1), self.time);
            self.magnetic_heading.push(payload.loc(17, 2), self.time);
            self.true_heading.push(payload.loc(17, 3), self.time);
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
    // let network_clone = network.clone();
    let task = pool.spawn(async move {
        // Blocking UDP Listener
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
            // if let Ok(record) = process_record(record_data) {
            //     payload.insert(record_index, record);
            // }
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
