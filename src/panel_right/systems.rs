use super::components::PanelRight;
// use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

pub fn spawn_panel_right(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(17.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        ..default()
                    },
                    z_index: ZIndex::Global(2),
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                PanelRight {},
            ));
        });
}
