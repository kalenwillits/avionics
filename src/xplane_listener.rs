use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task, TaskPool};
use futures_lite::future;
use std::collections::HashMap;
use std::convert::TryInto;
use std::net::UdpSocket;

const DEFAULT_ADDRESS: &str = "0.0.0.0:49000";

pub struct XPlaneListener;

type Record = HashMap<i32, f32>;
type Payload = HashMap<i32, Record>;

impl Plugin for XPlaneListener {
    fn build(&self, app: &mut App) {
        AsyncComputeTaskPool::init(|| TaskPool::new());
        app.insert_resource(Network {
            socket: UdpSocket::bind(DEFAULT_ADDRESS)
                .expect(format!("Failed to bind address {}", DEFAULT_ADDRESS).as_str()),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_task_system, resolve_task_system));
    }
}

#[derive(Resource)]
pub struct Network {
    socket: UdpSocket,
}

#[derive(Component, Default, Debug)]
pub struct AircraftState {
    pub pitch: Option<f32>,
    pub roll: Option<f32>,
    pub magnetic_heading: Option<f32>,
    pub true_heading: Option<f32>,
}

impl AircraftState {
    pub fn from(payload: Payload) -> Self {
        Self {
            pitch: get(&payload, 17, 0),
            roll: get(&payload, 17, 1),
            magnetic_heading: get(&payload, 17, 2),
            true_heading: get(&payload, 17, 3),
        }
    }
}

#[derive(Component)]
pub struct TaskSchedule;

#[derive(Component)]
pub struct TaskResult(Task<Result<Payload, ()>>);

fn setup(mut commands: Commands) {
    commands.spawn(TaskSchedule);
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
    mut task_queryset: Query<(Entity, &mut TaskResult)>,
    mut commands: Commands,
) {
    for (entity, mut task) in &mut task_queryset {
        if let Some(payload_result) = future::block_on(future::poll_once(&mut task.0)) {
            if let Ok(payload) = payload_result {
                commands.spawn(AircraftState::from(payload));
                commands.entity(entity).remove::<TaskResult>();
            }
        };
    }
}

pub fn get(payload: &Payload, i: i32, j: i32) -> Option<f32> {
    match payload.get(&i) {
        Some(record) => match record.get(&j) {
            Some(value) => Some(*value),
            None => None,
        },
        None => None,
    }
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
        for i in 0..num_records {
            let record_data = &raw[i * 36..(i + 1) * 36];
            // Extract the record number (first 4 bytes) and convert it to an integer
            let record_index = i32::from_le_bytes(
                record_data[..4]
                    .try_into()
                    .expect("Failed to read bytes while gathering record number!"),
            );
            if let Ok(record) = process_record(record_data) {
                payload.insert(record_index, record);
            }
        }
        return Ok(payload);
    }
    Err(())
}

fn process_record(record_data: &[u8]) -> Result<Record, ()> {
    let mut record: Record = Record::new();

    // Extract the record values (remaining 32 bytes)
    let record_values = &record_data[4..];

    // Iterate through the 8 values within the record
    for i in 0..8 {
        let offset = i * 4; // Offset to the start of each value (4 bytes each)
        let value = f32::from_le_bytes(
            record_values[offset..offset + 4]
                .try_into()
                .expect("Failed to read bytes when gathering record value!"),
        );
        record.insert(i as i32, value);
    }
    Ok(record)
}
