use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, TaskPool, Task};
use futures_lite::future;
use std::collections::HashMap;


pub struct XPlaneListener;

type Payload = HashMap<usize, HashMap<usize, f32>>;

impl Plugin for XPlaneListener {
    fn build(&self, app: &mut App) {
        AsyncComputeTaskPool::init(|| { TaskPool::new() } );
        app
            .insert_resource(AircraftState {..default()})
            // .insert_resource( ClientConfig {
            //     socket: UdpSocket::bind("0.0.0.0:49000").unwrap()
            // })
            .add_systems(Startup, setup)
            .add_systems(Update, (trigger_task_system, resolve_task_system));
    }
}

#[derive(Resource, Default)]
pub struct AircraftState {
    pitch: Option<f32>,
    roll: Option<f32>,
    magnetic_heading: Option<f32>,
    true_heading: Option<f32>,
}


#[derive(Component)]
pub struct TaskSchedule;

#[derive(Component)]
pub struct TaskResult(Task<Result<Payload, ()>>);

fn setup(mut commands: Commands) {
    commands.spawn(TaskSchedule);
}

pub fn spawn_task(
    commands: &mut Commands,
    target: Entity,
) {

    let pool = AsyncComputeTaskPool::get();
    let task = pool.spawn(async move {
        // Blocking Code
        println!("Blocking Code...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let payload: Payload = HashMap::new();
        Ok(payload)
    });
    commands.entity(target).insert(TaskResult(task));
}


pub fn trigger_task_system(
        mut commands: Commands,
        task_schedule_queryset: Query<Entity, (With<TaskSchedule>, Without<TaskResult>)>,
    ) {
        for entity in &task_schedule_queryset {
            spawn_task(&mut commands, entity);
        }

}


pub fn resolve_task_system(
    mut task_queryset: Query<(Entity, &mut TaskResult)>,
    mut commands: Commands,
) {
    for (entity, mut task) in &mut task_queryset {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // Resolve logic
            commands.entity(entity).remove::<TaskResult>();
        };
    };
}

