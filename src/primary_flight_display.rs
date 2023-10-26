use crate::utils::degrees_to_radians;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const NUM_PITCH_LINES: usize = 16;

pub struct PrimaryFlightDisplay;

impl Plugin for PrimaryFlightDisplay {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_crosshairs,
                spawn_artifical_horizon,
                spawn_pitch_lines,
                spawn_airspeed_indicator,
            ),
        )
        .add_systems(
            Update,
            (
                update_artifical_horizon_system,
                update_pitch_lines_system,
                update_airspeed_indicator,
            ),
        );
    }
}

#[derive(Component)]
struct ArtificalHorizon;

#[derive(Component)]
struct PitchLines;

#[derive(Component)]
struct AirspeedIndicator;

fn spawn_artifical_horizon(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform {
                    scale: (1.6, 1.7, 1.0).into(),
                    ..default()
                },
                ..default()
            },
            Name::new("ArtificialHorizon"),
            ArtificalHorizon {},
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    transform: Transform {
                        scale: (1.0, 2.0, 1.0).into(),
                        ..default()
                    },
                    background_color: Color::rgb(0.0, 0.4, 0.8).into(),
                    ..default()
                },
                Name::new("AboveHorizon"),
            ));

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        border: UiRect::top(Val::Px(1.0)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
                    ..default()
                },
                Name::new("BelowHorizon"),
            ));
        });
}

fn spawn_pitch_lines(mut commands: Commands) {
    commands
        .spawn((
            Name::new("PitchLines"),
            PitchLines {},
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
        ))
        .with_children(|parent| {
            for i in 1..NUM_PITCH_LINES {
                let height: f32 = i as f32 * 5.0;
                let width: f32;
                if (i as f32 * 2.5) % 2.0 == 0.0 {
                    width = 10.0;
                } else if (i as f32 * 2.5) % 1.0 == 0.0 {
                    width = 5.0;
                } else {
                    width = 2.5;
                };
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(width),
                        height: Val::Percent(height),
                        align_items: AlignItems::Center,
                        border: UiRect {
                            top: Val::Px(1.0),
                            bottom: Val::Px(1.0),
                            ..default()
                        },
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    z_index: ZIndex::Local(2),
                    border_color: Color::WHITE.into(),
                    ..default()
                });
            }
        });
}

fn spawn_crosshairs(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            },
            Name::new("Crosshairs"),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(512.0),
                        top: Val::Px(15.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            border: UiRect {
                                top: Val::Px(6.0),
                                right: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        },
                        border_color: Color::rgb(0.7, 0.6, 0.0).into(),
                        ..default()
                    });
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            top: Val::Px(-5.0),
                            border: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        border_color: Color::rgb(0.7, 0.6, 0.0).into(),
                        ..default()
                    });
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            border: UiRect {
                                top: Val::Px(6.0),
                                left: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        },
                        border_color: Color::rgb(0.7, 0.6, 0.0).into(),
                        ..default()
                    });
                });
        });
}

fn spawn_airspeed_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(2),
                ..default()
            },
            Name::new("IndicatorLayers"),
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(16.666),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(33.3333),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(1.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Name::new("DigitalDisplay"),
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(66.0),
                                    height: Val::Px(36.0),
                                    border: UiRect::all(Val::Px(1.0)),
                                    // flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::End,
                                    ..default()
                                },

                                background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                AirspeedIndicator {},
                                TextBundle::from_section(
                                    "----",
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/ubuntu_mono/UbuntuMono-Bold.ttf"),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    flex_grow: -1.0,
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::Right),
                            ));
                        });
                });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(33.3333),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(16.6666),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                ..default()
            });
        });
}

fn update_artifical_horizon_system(
    aircraft_state: Res<AircraftState>,
    mut artifical_horizon_queryset: Query<(&mut Transform, &mut Style), With<ArtificalHorizon>>,
) {
    let (mut artifical_horizon_transform, mut artifical_horizon_style) =
        artifical_horizon_queryset.single_mut();
    artifical_horizon_transform.rotation.z = degrees_to_radians(aircraft_state.roll) * -0.5;
    artifical_horizon_style.top = Val::Percent(aircraft_state.pitch);
}

fn update_pitch_lines_system(
    aircraft_state: Res<AircraftState>,
    mut pitch_lines_queryset: Query<(&mut Transform, &mut Style), With<PitchLines>>,
) {
    let (mut pitch_lines_transform, mut pitch_lines_style) = pitch_lines_queryset.single_mut();
    pitch_lines_transform.rotation.z = degrees_to_radians(aircraft_state.roll) * -0.5;
    pitch_lines_style.top = Val::Percent(aircraft_state.pitch);
}

fn update_airspeed_indicator(
    aircraft_state: Res<AircraftState>,
    mut airspeed_indicator_queryset: Query<&mut Text, With<AirspeedIndicator>>,
) {
    let mut airspeed_indicator_text = airspeed_indicator_queryset.single_mut();
    let value: f32 = aircraft_state.indicated_airspeed.round();
    if value == 0.0 {
        airspeed_indicator_text.sections[0].value = format!("----");
    } else {
        airspeed_indicator_text.sections[0].value = format!("{}", value);
    }
}
