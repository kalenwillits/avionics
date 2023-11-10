use super::components::{
    VerticalSpeedIndicator, VerticalSpeedIndicatorDigital, VerticalSpeedIndicatorNeedle,
    VerticalSpeedIndicatorArrow
};
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

const NUM_LINES: i32 = 5;
const LINE_DISTANCE: f32 = 25.0;
const MAX_VERTICAL_SPEED: f32 = 2000.0;
const RANGE_FACTOR: f32 = 50.0;
const DIGITAL_DISPLAY_HEIGHT: f32 = 12.0;
const RIBBON_WIDTH: f32 = 36.0;
const UP_ARROW: &str = "+";
const DOWN_ARROW: &str = "-";


pub fn spawn_vertical_speed_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                    width: Val::Px(RIBBON_WIDTH),
                                    height: Val::Px(270.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::top(Val::Px(1.0)),
                                    overflow: Overflow::clip(),
                                    align_items: AlignItems::Start,
                                    ..default()
                                },
                                background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    VerticalSpeedIndicatorNeedle {},
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Px(RIBBON_WIDTH),
                                            height: Val::Px(2.0),
                                            ..default()
                                        },
                                        z_index: ZIndex::Local(3),
                                        background_color: Color::WHITE.into(),
                                        ..default()
                                    },
                                ))
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            Name::new("DigitalDisplay"),
                                            NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(100.0),
                                                    height: Val::Px(DIGITAL_DISPLAY_HEIGHT),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    flex_direction: FlexDirection::Column,
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                VerticalSpeedIndicatorDigital {},
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/ubuntu_mono/UbuntuMono-Regular.ttf"),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::Center),
                            ));
                            parent.spawn((
                                VerticalSpeedIndicatorArrow {},
                                TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server
                                            .load("fonts/ubuntu_mono/UbuntuMono-Regular.ttf"),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    ..default()
                                })
                                .with_text_alignment(TextAlignment::Center),
                            ));
                                        });
                                    parent.spawn((
                                        Name::new("HeadingIndicatorCircle"),
                                        NodeBundle {
                                            style: Style {
                                                // TODO - Add in HSI graphic
                                                height: Val::Percent(25.0),
                                                width: Val::Percent(25.0),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                    ));
                                });

                            for i in 0..NUM_LINES {
                                let height: f32 = i as f32 * LINE_DISTANCE;
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Px(RIBBON_WIDTH),
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
    mut vertical_speed_indicator_digital_queryset: Query<
        (&mut Text, &mut Style),
        (
            With<VerticalSpeedIndicatorDigital>,
            Without<VerticalSpeedIndicatorNeedle>,
            Without<VerticalSpeedIndicatorArrow>,
        ),
    >,
    mut vertical_speed_indicator_arrow_queryset: Query<
        (&mut Text, &mut Style),
        (
            With<VerticalSpeedIndicatorArrow>,
            Without<VerticalSpeedIndicatorNeedle>,
            Without<VerticalSpeedIndicatorDigital>,
        ),
    >,

) {
    let mut style = vertical_speed_indicator_queryset.single_mut();
    style.top = Val::Percent(
        (-(aircraft_state.vertical_speed / (MAX_VERTICAL_SPEED / RANGE_FACTOR))).clamp(-49.0, 49.0),
    );
    let (mut digital_text, mut digital_style) = vertical_speed_indicator_digital_queryset.single_mut();
    let (mut arrow_text, mut arrow_style) = vertical_speed_indicator_arrow_queryset.single_mut();
    let value: i32 = ((aircraft_state.vertical_speed / 10.0).round() * 10.0) as i32;
    if value.abs() >= 100 {
        digital_text.sections[0].value = format!("{}", value.abs());
        if value <= 0 {
            arrow_style.top = Val::Px(-DIGITAL_DISPLAY_HEIGHT * 4.0);
            arrow_text.sections[0].value = DOWN_ARROW.to_string();
            digital_style.top = Val::Px(-DIGITAL_DISPLAY_HEIGHT); 
        } else {
            arrow_style.top = Val::Px(DIGITAL_DISPLAY_HEIGHT);
            arrow_text.sections[0].value = UP_ARROW.to_string();
            digital_style.top = Val::Px(DIGITAL_DISPLAY_HEIGHT);
        }
    } else {
      arrow_text.sections[0].value = "".to_string();
      digital_text.sections[0].value = "".to_string();
    }
}
