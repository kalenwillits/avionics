use super::components::PanelLeft;
// use crate::xplane_listener::AircraftState;
use bevy::prelude::*;


pub fn spawn_panel_left(mut commands: Commands) {
    commands
        .spawn(
            NodeBundle {
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
            },
        ).with_children(|parent| {
            parent.spawn((NodeBundle {
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
                )).with_children(|parent| {
                // Spawn ECAS here
            });
        });
}
