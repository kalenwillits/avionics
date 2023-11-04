use super::components::{VerticalSpeedIndicator, VerticalSpeedIndicatorNeedle};
use crate::xplane_listener::AircraftState;
use crate::utils::degrees_to_radians;
use bevy::prelude::*;

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
                parent.spawn((
                    VerticalSpeedIndicator {},
                    NodeBundle {
                        style: Style {
                            width: Val::Px(36.0),
                            height: Val::Px(270.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                        ..default()
                    }
                ))
                    .with_children(|parent| {
                        parent.spawn((
                            VerticalSpeedIndicatorNeedle {},
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(36.0),
                                    height: Val::Px(2.0),
                                    margin: UiRect::top(Val::Px(1.0)),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            }
                        ));
                    });
            });
        });
}

pub fn update_vertical_speed_indicator(
    aircraft_state: Res<AircraftState>,
    mut vertical_speed_indicator_queryset: Query<(&mut Transform, &mut Style), With<VerticalSpeedIndicatorNeedle>>,
) {
    let (mut transform, mut style) = vertical_speed_indicator_queryset.single_mut();
    style.top = Val::Percent(0.1);
    // if vertical_speed_indicator_needle_transform.rotation.z > degrees_to_radians(25.0) {
    //     vertical_speed_indicator_needle_transform.rotation.z = 0.0;
    // };
    // vertical_speed_indicator_needle_transform.rotation.z = (vertical_speed_indicator_needle_transform.rotation.z + degrees_to_radians(0.1)).clamp(degrees_to_radians(0.0), degrees_to_radians(30.0));
    // let value: f32 = aircraft_state.magnetic_heading.round();
    // vertical_speed_indicator_text.sections[0].value = format!("{}", value);
}
