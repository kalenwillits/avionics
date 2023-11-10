use super::components::{TurnCoordinator, TurnCoordinatorBall};
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const MAX_RANGE: f32 = 0.01;
const INSTRUMENT_SIZE: f32 = 96.0;
const BALL_SIZE: f32 = 18.0;

pub fn spawn_turn_coordinator(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    ..default()
                },
                z_index: ZIndex::Local(2),
                ..default()
            },
            Name::new("IndicatorLayers"),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(27.333),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            TurnCoordinator {},
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(INSTRUMENT_SIZE),
                                    height: Val::Px(24.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    overflow: Overflow::clip(),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TurnCoordinatorBall {},
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(BALL_SIZE),
                                        height: Val::Px(BALL_SIZE),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    ..default()
                                },
                            ));
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(1.0),
                                    height: Val::Px(20.0),
                                    left: Val::Px((INSTRUMENT_SIZE / 2.0) - 14.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Px(1.0),
                                    height: Val::Px(20.0),
                                    right: Val::Px((INSTRUMENT_SIZE / 2.0) - 14.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            });
                        });
                });
        });
}

pub fn update_turn_coordinator(
    aircraft_state: Res<AircraftState>,
    mut turn_coordinator_queryset: Query<&mut Style, With<TurnCoordinatorBall>>,
) {
    let mut style = turn_coordinator_queryset.single_mut();
    style.left = Val::Percent((50.0 - (BALL_SIZE / 2.0)) - (aircraft_state.gload_side / MAX_RANGE));
}
