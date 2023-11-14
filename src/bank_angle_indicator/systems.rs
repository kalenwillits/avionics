use super::components::{BankAngleIndicator, BankAngleLine, BankAngleNeedle};
use crate::utils::degrees_to_radians;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const BANK_ANGLE_RADIUS: f32 = 33.3;
const BANK_ANGLE_TICK_WIDTH: f32 = 1.0;
const BANK_ANGLE_TICK_SIZE: f32 = 3.3;
const BANK_ANGLE_DEGREES: [f32; 11] = [
    -60.0, -45.0, -30.0, -20.0, -10.0, 0.0, 10.0, 20.0, 30.0, 45.0, 60.0,
];

pub fn spawn_bank_angle_indicator(mut commands: Commands) {
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
                z_index: ZIndex::Local(10),
                ..default()
            },
            Name::new("BankAngleIndicator"),
            BankAngleIndicator {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    BankAngleNeedle {},
                    NodeBundle {
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
                    },
                ))
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

            for degree in BANK_ANGLE_DEGREES.iter() {
                parent
                    .spawn((
                        BankAngleLine {},
                        NodeBundle {
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
                        },
                    ))
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
                    })
                    .with_children(|parent| {
                        parent.spawn(NodeBundle {
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
                        });
                    });
            }
        });
}

pub fn update_bank_angle_lines(
    aircraft_state: Res<AircraftState>,
    mut pitch_lines_queryset: Query<&mut Transform, With<BankAngleLine>>,
) {
    for mut pitch_line_transform in pitch_lines_queryset.iter_mut() {
        pitch_line_transform.rotation.z = degrees_to_radians(aircraft_state.roll) * -0.5;
    }
}
