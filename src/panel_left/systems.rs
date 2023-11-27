use super::components::{EngineOne, PanelLeft, TachometerNeedle, TachometerValue};
use crate::database::resources::Database;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;
use sqlite;

pub fn spawn_panel_left(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    database: Res<Database>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(17.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        z_index: ZIndex::Global(2),
                        background_color: Color::BLACK.into(),
                        ..default()
                    },
                    PanelLeft {},
                ))
                .with_children(|parent| {
                    let mut statement = database
                        .connection
                        .prepare(
                            "
                            SELECT * FROM ENGINE WHERE AIRCRAFT IN (SELECT AIRCRAFT FROM CONFIG)
                        ",
                        )
                        .unwrap();
                    let mut engine_num: usize = 0;
                    while let Ok(sqlite::State::Row) = statement.next() {
                        engine_num += 1;
                        let min_rpm = statement.read::<i64, _>("RPM_MIN").unwrap();
                        let max_rpm = statement.read::<i64, _>("RPM_MAX").unwrap();
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(96.0),
                                    height: Val::Px(32.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Start,
                                    align_items: AlignItems::Start,
                                    margin: UiRect::all(Val::Px(4.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn((NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::SpaceEvenly,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },))
                                    .with_children(|parent| {
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    justify_content: JustifyContent::Start,
                                                    align_items: AlignItems::Start,
                                                    width: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                "RPM",
                                                TextStyle {
                                                    font: asset_server.load(
                                                        "fonts/ubuntu_mono/UbuntuMono-Regular.ttf",
                                                    ),
                                                    font_size: 24.0,
                                                    color: Color::WHITE,
                                                    ..default()
                                                },
                                            ));
                                            });
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    justify_content: JustifyContent::End,
                                                    align_items: AlignItems::End,
                                                    width: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                let mut tachometer_entity = parent.spawn((
                                        TachometerValue {},
                                        TextBundle::from_section(
                                            "---",
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/ubuntu_mono/UbuntuMono-Regular.ttf",
                                                ),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        ),
                                    ));
                                                if engine_num == 1 {
                                                    tachometer_entity.insert(EngineOne {});
                                                };
                                            });
                                    });
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(4.0),
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Start,
                                            ..default()
                                        },
                                        background_color: Color::GRAY.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        let mut tachometer_needle_entity = parent.spawn((
                                            TachometerNeedle {},
                                            NodeBundle {
                                                style: Style {
                                                    height: Val::Px(8.0),
                                                    width: Val::Px(4.0),
                                                    position_type: PositionType::Absolute,
                                                    ..default()
                                                },
                                                background_color: Color::WHITE.into(),
                                                ..default()
                                            },
                                        ));
                                        if engine_num == 1 {
                                            tachometer_needle_entity.insert(EngineOne {});
                                        };
                                    });
                            });
                    }
                });
        });
}

pub fn update_engine_one_tachometer(
    aircraft_state: Res<AircraftState>,
    mut engine_one_tachometer_value_queryset: Query<
        &mut Text,
        (With<TachometerValue>, With<EngineOne>),
    >,
    // mut tachometer_needle_queryset: Query<&mut Style, With<TachometerNeedle>>,
) {
    for mut tachometer_value_text in engine_one_tachometer_value_queryset.iter_mut() {
        let value: f32 = aircraft_state.engine_rpm.round();
        tachometer_value_text.sections[0].value = format!("{}", value);
    }

    // TODO - Aircraft min/max from profile.
    // let mut tachometer_needle_style = tachometer_needle_queryset.single_mut();
    // tachometer_needle_style.left = Val::Px(50.0);
}
