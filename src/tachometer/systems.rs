use super::components::Tachometer;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const MAX_RANGE: f32 = 0.01;
const INSTRUMENT_SIZE: f32 = 96.0;
const BALL_SIZE: f32 = 18.0;

pub fn spawn_tachometer(mut commands: Commands, _asset_server: Res<AssetServer>) {
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
                        height: Val::Percent(33.333),
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
                            Tachometer {},
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
                            parent.spawn(
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
                            );
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

pub fn update_tachometer(
    aircraft_state: Res<AircraftState>,
    mut tachometer_queryset: Query<&mut Style>,
) {
    let mut style = tachometer_queryset.single_mut();
}
