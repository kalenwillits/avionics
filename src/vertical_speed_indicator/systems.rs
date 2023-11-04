use super::components::{VerticalSpeedIndicator, VerticalSpeedIndicatorNeedle};
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const NUM_LINES: i32 = 5;
const LINE_DISTANCE: f32 = 25.0;
const MAX_VERTICAL_SPEED: f32 = 2000.0;
const RANGE_FACTOR: f32 = 50.0;

pub fn spawn_vertical_speed_indicator(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::End,
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
                        width: Val::Percent(27.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            VerticalSpeedIndicator {},
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(36.0),
                                    height: Val::Px(270.0),
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
                                VerticalSpeedIndicatorNeedle {},
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(36.0),
                                        height: Val::Px(4.0),
                                        margin: UiRect::top(Val::Px(1.0)),
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.7, 0.6, 0.0).into(),
                                    ..default()
                                },
                            ));
                            for i in 0..NUM_LINES {
                                let height: f32 = i as f32 * LINE_DISTANCE;
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(36.0),
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
                                    border_color: Color::GRAY.into(),
                                    ..default()
                                });
                            }
                        });
                });
        });
}

pub fn update_vertical_speed_indicator(
    aircraft_state: Res<AircraftState>,
    mut vertical_speed_indicator_queryset: Query<&mut Style, With<VerticalSpeedIndicatorNeedle>>,
) {
    let mut style = vertical_speed_indicator_queryset.single_mut();
    style.top = Val::Percent(
        (-(aircraft_state.vertical_speed / (MAX_VERTICAL_SPEED / RANGE_FACTOR))).clamp(-49.0, 49.0),
    );
}
