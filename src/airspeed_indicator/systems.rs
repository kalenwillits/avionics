use bevy::prelude::*;
use crate::xplane_listener::AircraftState;
use super::components::AirSpeedIndicator;


pub fn spawn_airspeed_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                AirSpeedIndicator {},
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


pub fn update_airspeed_indicator(
    aircraft_state: Res<AircraftState>,
    mut airspeed_indicator_queryset: Query<&mut Text, With<AirSpeedIndicator>>,
) {
    let mut airspeed_indicator_text = airspeed_indicator_queryset.single_mut();
    let value: f32 = aircraft_state.indicated_airspeed.round();
    if value == 0.0 {
        airspeed_indicator_text.sections[0].value = format!("----");
    } else {
        airspeed_indicator_text.sections[0].value = format!("{}", value);
    }
}
