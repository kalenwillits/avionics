use bevy::prelude::*;
use crate::xplane_listener::AircraftState;


pub struct PrimaryFlightDisplay;

impl Plugin for PrimaryFlightDisplay {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                    spawn_crosshairs,
                    spawn_artifical_horizon,
                ));
            // .add_systems(Update, update_attitide_indicator_roll_system);
    }
}

#[derive(Component)]
struct ArtificalHorizon;



fn spawn_artifical_horizon(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform {
                    scale: (1.6, 1.7, 1.0).into(),
                    ..default()
                },
                ..default()
            }, 
            Name::new("ArtificialHorizon"),
            ArtificalHorizon {},
            ))
    .with_children(|parent| {
        parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    transform: Transform {
                        scale: (1.0, 2.0, 1.0).into(),
                        ..default()
                    },
                    background_color: Color::rgb(0.0, 0.5, 1.0).into(),
                    ..default()
                },
                Name::new("AboveHorizon")
        ));

        parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        border: UiRect::top(Val::Px(2.0)),
                        ..default()
                    },
                    transform: Transform {
                        scale: (1.0, 1.0, 1.0).into(),
                        ..default()
                    },
                    background_color: Color::rgb(0.25, 0.25, 0.25).into(),
                    border_color: Color::WHITE.into(),
                    ..default()
                },
                Name::new("BelowHorizon")
        ));


        });

}



fn spawn_crosshairs(
        mut commands: Commands
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                z_index: ZIndex::Local(1),
                ..default()
            }, 
            Name::new("Crosshairs")
            )).with_children(|parent| {
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            width: Val::Px(512.0),
                            top: Val::Px(15.0),
                            justify_content: JustifyContent::SpaceEvenly,
                            ..default()
                        },
                        ..default()
                    }
                ).with_children(|parent| {
                    parent.spawn(
                    NodeBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            border: UiRect {
                                top: Val::Px(6.0),
                                right: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        },
                        border_color: Color::YELLOW.into(),
                        ..default()
                    });
                    parent.spawn(
                    NodeBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            top: Val::Px(-5.0),
                            border: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        border_color: Color::YELLOW.into(),
                        ..default()
                    });
                    parent.spawn(
                    NodeBundle {
                        style: Style {
                            width: Val::Px(96.0),
                            height: Val::Px(32.0),
                            border: UiRect {
                                top: Val::Px(6.0),
                                left: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        },
                        border_color: Color::YELLOW.into(),
                        ..default()
                    });

                });

        });
}

// fn update_attitide_indicator_roll_system(aircraft_state: Res<AircraftState>, mut artifical_horizon_queryset: Query<&mut Transform, With<ArtificalHorizon>>) {
//     // let mut artifical_horizon_transform = artifical_horizon_queryset.single();
//     for mut transform in artifical_horizon_queryset.iter_mut() {
//         // transform.rotation.z = aircraft_state.roll;
//         // let radians: f32 = transform.rotation.z - aircraft_state.roll;
//         // transform.rotate_z(radians);
//     }
//     // let radians: f32 = artifical_horizon_transform.rotation.z - aircraft_state.roll;
//     // artifical_horizon_transform.rotate_z(radians);
// }
