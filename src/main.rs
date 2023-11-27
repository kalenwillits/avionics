mod airspeed_indicator;
mod altimeter;
mod artificial_horizon;
mod bank_angle_indicator;
mod heading_indicator;
mod panel_left;
mod panel_right;
mod turn_coordinator;
mod vertical_speed_indicator;

mod database;
mod utils;
mod xplane_listener;

use bevy::prelude::*;
use database::resources::Database;

const STEAM_DECK_RESOLUTION: (f32, f32) = (1280f32, 800f32);

fn main() {
    let database = database::connect();
    App::new()
        .insert_resource(database)
        .add_systems(Startup, mock_data.before(setup))
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Avionics".into(),
                    resolution: STEAM_DECK_RESOLUTION.into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            xplane_listener::XPlaneListener,
            artificial_horizon::ArtificialHorizonPlugin,
            bank_angle_indicator::BankAngleIndicatorPlugin,
            airspeed_indicator::AirSpeedIndicatorPlugin,
            vertical_speed_indicator::VerticalSpeedIndicatorPlugin,
            turn_coordinator::TurnCoordinatorPlugin,
            altimeter::AltimeterPlugin,
            heading_indicator::HeadingIndicatorPlugin,
            panel_left::PanelLeftPlugin,
            panel_right::PanelRightPlugin,
        ))
        .run();
}

fn mock_data(database: Res<Database>) {
    // Temporary hard coded configuration settings ---------------------------
    database.connection.execute("DELETE FROM CONFIG").unwrap();
    database.connection.execute("DELETE FROM AIRCRAFT").unwrap();
    database.connection.execute("DELETE FROM PILOT").unwrap();
    database.connection.execute("DELETE FROM ENGINE").unwrap();
    database
        .connection
        .execute("INSERT INTO PILOT ( NAME ) VALUES ( 'TestPilot' )")
        .unwrap();
    database
        .connection
        .execute("INSERT INTO AIRCRAFT ( CALLSIGN, MAKE, MODEL) VALUES ( 'N135TS', 'Vans', 'RV12')")
        .unwrap();
    database.connection.execute("INSERT INTO ENGINE ( MAKE, MODEL, RPM_MIN, RPM_MAX, NORMAL_OPERATING_MIN, NORMAL_OPERATING_MAX, AIRCRAFT) VALUES ( 'Rotax', '912ULS', 0, 5800, 1400, 5300, (SELECT MAX(PK) FROM AIRCRAFT))").unwrap();
    // -----------------------------------------------------------------------
}

fn setup(mut commands: Commands, database: Res<Database>) {
    commands.spawn(Camera2dBundle::default());
    if let Some(config_result) = database
        .connection
        .prepare("SELECT COUNT(1) AS config_count FROM CONFIG")
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
        .next()
    {
        let config_count = config_result.read::<i64, _>("config_count");
        if config_count == 0 {
            database
                .connection
                .execute(
                    "INSERT INTO CONFIG (
                AIRCRAFT, 
                PILOT
                ) VALUES (
                    (SELECT MIN(PK) FROM AIRCRAFT),
                    (SELECT MIN(PK) FROM PILOT)
                )",
                )
                .unwrap();
        } else if config_count > 1 {
            database.connection.execute("DELETE FROM CONFIG WHERE PK IN (
                        SELECT PK FROM CONFIG WHERE PK<>(SELECT PK FROM CONFIG ORDER BY PK DESC LIMIT 1)
                    )").unwrap();
        }
    };
}
