use bevy::prelude::*;


pub struct PrimaryFlightDisplay;

impl Plugin for PrimaryFlightDisplay {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ui);
    }
}



fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            }, 
            Name::new("ui")
            ))
    .with_children(|parent| {
        parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(120.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLUE.into(),
                    ..default()
                },
                Name::new("BlueHorizon")
        ));

        parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(120.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::ORANGE.into(),
                    ..default()
                },
                Name::new("BrownHorizon")
        ));




    });

}
