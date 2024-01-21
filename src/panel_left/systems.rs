use super::components::{EngineOne, PanelLeft, TachometerNeedle, TachometerValue};
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;
use crate::profiles::resources::Profile;

pub fn spawn_panel_left(
        mut commands: Commands, 
        asset_server: Res<AssetServer>,
        profile: Res<Profile>,

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

                        // let rpm_min = statement.read::<i64, _>("RPM_MIN").unwrap();
                        // let rpm_max = statement.read::<i64, _>("RPM_MAX").unwrap();
                        // let normal_operating_min =
                            // statement.read::<i64, _>("NORMAL_OPERATING_MIN").unwrap();
                        // let normal_operating_max =
                            // statement.read::<i64, _>("NORMAL_OPERATING_MAX").unwrap();
                        let engine_index: usize = 1;
                        let engine = match engine_index {
                            1 => EngineOne {
                                rpm_min: profile.engine_one.range.min,
                                rpm_max: profile.engine_one.range.max,
                                normal_operating_min: profile.engine_one.normal.min,
                                normal_operating_max: profile.engine_one.normal.max,
                            },
                            _ => panic!("Unsupported number of engines!"),
                        };
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
                                                parent.spawn((
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
                                    )).insert(engine);
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
                                        parent.spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(
                                                    ((engine.normal_operating_max as f32
                                                        / engine.rpm_max as f32)
                                                        - (engine.normal_operating_min as f32
                                                            / engine.rpm_max as f32))
                                                        * 100.0,
                                                ),
                                                height: Val::Px(4.0),
                                                left: Val::Percent(
                                                    (engine.normal_operating_min as f32 / engine.rpm_max as f32)
                                                        * 100.0,
                                                ),
                                                flex_direction: FlexDirection::Column,
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Start,
                                                position_type: PositionType::Absolute,
                                                ..default()
                                            },
                                            background_color: Color::GREEN.into(),
                                            ..default()
                                        });

                                        parent
                                            .spawn((
                                                TachometerNeedle {},
                                                NodeBundle {
                                                    style: Style {
                                                        height: Val::Px(8.0),
                                                        width: Val::Px(4.0),
                                                        position_type: PositionType::Absolute,
                                                        ..default()
                                                    },
                                                    z_index: ZIndex::Global(4),
                                                    background_color: Color::WHITE.into(),
                                                    ..default()
                                                },
                                            ))
                                            .insert(engine);
                                    });
                            });
                });
        });
}

pub fn update_engine_one_tachometer(
    aircraft_state: Res<AircraftState>,
    mut engine_one_tachometer_value_queryset: Query<
        (&mut Text, &EngineOne),
        (With<TachometerValue>, With<EngineOne>),
    >,
    mut engine_one_tachometer_needle_queryset: Query<
        (&mut Style, &EngineOne),
        (With<TachometerNeedle>, With<EngineOne>),
    >,
) {
    let value: f32 = aircraft_state.engine_rpm.round();

    for (mut tachometer_value_text, _engine_one) in engine_one_tachometer_value_queryset.iter_mut()
    {
        tachometer_value_text.sections[0].value = format!("{}", value);
    }
    for (mut tachometer_needle_style, engine_one) in
        engine_one_tachometer_needle_queryset.iter_mut()
    {
        tachometer_needle_style.left = Val::Percent((value / engine_one.rpm_max as f32) * 100.0);
    }
}
