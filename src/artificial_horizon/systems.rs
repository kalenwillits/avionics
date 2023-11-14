use super::components::{ArtificialHorizon, PitchLines};
use crate::utils::degrees_to_radians;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const NUM_PITCH_LINES: usize = 16;
const BANK_ANGLE_RADIUS: f32 = 33.3;
const BANK_ANGLE_TICK_WIDTH: f32 = 1.0;
const BANK_ANGLE_TICK_SIZE: f32 = 3.3;
const BANK_ANGLE_DEGREES: [f32; 11] = [
    -67.0, -45.0, -30.0, -20.0, -10.0, 0.0, 10.0, 20.0, 30.0, 45.0, 67.0,
];

pub fn spawn_artificial_horizon(mut commands: Commands) {
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
            ArtificialHorizon {},
        ))
        .with_children(|parent| {
            for degree in BANK_ANGLE_DEGREES.iter() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        z_index: ZIndex::Local(10),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(BANK_ANGLE_TICK_WIDTH),
                                    height: Val::Percent(BANK_ANGLE_RADIUS),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Start,
                                    justify_content: JustifyContent::Start,
                                    ..default()
                                },
                                transform: Transform {
                                    rotation: Quat::from_rotation_z(degrees_to_radians(*degree)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        height: Val::Percent(BANK_ANGLE_TICK_SIZE),
                                        width: Val::Percent(100.0),
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                });
                            });
                    });
            }
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
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    z_index: ZIndex::Local(10),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(BANK_ANGLE_TICK_WIDTH),
                                height: Val::Percent(BANK_ANGLE_RADIUS - BANK_ANGLE_TICK_SIZE),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::Start,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    height: Val::Percent(BANK_ANGLE_TICK_SIZE * 2.0),
                                    width: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            });
                        });
                });
        });
}

pub fn spawn_pitch_lines(mut commands: Commands) {
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

pub fn spawn_crosshairs(mut commands: Commands) {
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

pub fn update_artificial_horizon(
    aircraft_state: Res<AircraftState>,
    mut artificial_horizon_queryset: Query<(&mut Transform, &mut Style), With<ArtificialHorizon>>,
) {
    let (mut artificial_horizon_transform, mut artificial_horizon_style) =
        artificial_horizon_queryset.single_mut();
    artificial_horizon_transform.rotation.z = degrees_to_radians(aircraft_state.roll) * -0.5;
    artificial_horizon_style.top = Val::Percent(aircraft_state.pitch);
}

pub fn update_pitch_lines(
    aircraft_state: Res<AircraftState>,
    mut pitch_lines_queryset: Query<(&mut Transform, &mut Style), With<PitchLines>>,
) {
    let (mut pitch_lines_transform, mut pitch_lines_style) = pitch_lines_queryset.single_mut();
    pitch_lines_transform.rotation.z = degrees_to_radians(aircraft_state.roll) * -0.5;
    pitch_lines_style.top = Val::Percent(aircraft_state.pitch);
}
