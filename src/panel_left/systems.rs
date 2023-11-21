use super::components::{PanelLeft, TachometerValue};
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

pub fn spawn_panel_left(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(98.0),
                                height: Val::Px(32.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Start,
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(80.0),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },))
                                .with_children(|parent| {
                                    parent.spawn(
                                        TextBundle::from_section(
                                            "RPM",
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/ubuntu_mono/UbuntuMono-Regular.ttf",
                                                ),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                // flex_grow: -1.0,
                                                ..default()
                                            },
                                        ),
                                    );
                                    parent.spawn((
                                        TachometerValue {},
                                        TextBundle::from_section(
                                            "0",
                                            TextStyle {
                                                font: asset_server.load(
                                                    "fonts/ubuntu_mono/UbuntuMono-Regular.ttf",
                                                ),
                                                font_size: 24.0,
                                                color: Color::WHITE,
                                                ..default()
                                            },
                                        )
                                        .with_style(
                                            Style {
                                                // flex_grow: -1.0,
                                                ..default()
                                            },
                                        ),
                                    ));
                                });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(4.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Start,
                                    align_items: AlignItems::Start,
                                    ..default()
                                },
                                background_color: Color::GRAY.into(),
                                ..default()
                            });
                        });
                });
        });
}

pub fn update_tachometer(
    aircraft_state: Res<AircraftState>,
    mut tachometer_value_queryset: Query<&mut Text, With<TachometerValue>>,
) {
    let mut tachometer_value_text = tachometer_value_queryset.single_mut();
    let value: f32 = aircraft_state.engine_rpm.round();
    tachometer_value_text.sections[0].value = format!("{}", value);
}
