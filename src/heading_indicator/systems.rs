use super::components::HeadingIndicator;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

pub fn spawn_heading_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
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
                            Name::new("DigitalDisplay"),
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(66.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                HeadingIndicator {},
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
        });
}

pub fn update_heading_indicator(
    aircraft_state: Res<AircraftState>,
    mut heading_indicator_queryset: Query<&mut Text, With<HeadingIndicator>>,
) {
    let mut heading_indicator_text = heading_indicator_queryset.single_mut();
    let value: f32 = aircraft_state.magnetic_heading.round();
    heading_indicator_text.sections[0].value = format!("{}", value);
}
