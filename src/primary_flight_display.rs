use crate::utils::degrees_to_radians;
use crate::xplane_listener::AircraftState;
use bevy::prelude::*;

pub struct PrimaryFlightDisplay;

impl Plugin for PrimaryFlightDisplay {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_crosshairs, spawn_artifical_horizon))
            .add_systems(Update, consume_aircraft_state_system);
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
                Name::new("AboveHorizon"),
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
                Name::new("BelowHorizon"),
            ));
        });
}

fn spawn_crosshairs(mut commands: Commands) {
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
            Name::new("Crosshairs"),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(512.0),
                        top: Val::Px(15.0),
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
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
                        border_color: Color::rgb(1.0, 0.5, 0.0).into(),
                        ..default()
                    });
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            top: Val::Px(-5.0),
                            border: UiRect::all(Val::Px(6.0)),
                            ..default()
                        },
                        border_color: Color::rgb(1.0, 0.5, 0.0).into(),
                        ..default()
                    });
                    parent.spawn(NodeBundle {
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
                        border_color: Color::rgb(1.0, 0.5, 0.0).into(),
                        ..default()
                    });
                });
        });
}

fn consume_aircraft_state_system(
    mut commands: Commands,
    mut aircraft_state_queryset: Query<(Entity, &mut AircraftState)>,
    mut artifical_horizon_queryset: Query<&mut Transform, With<ArtificalHorizon>>,
) {
    for (entity, aircraft_state) in aircraft_state_queryset.iter_mut() {
        let mut transform = artifical_horizon_queryset.single_mut();
        // TODO - interpolate
        if let Some(roll) = aircraft_state.roll {
            transform.rotation.z = degrees_to_radians(roll) * -0.5;
        }
        commands.entity(entity).despawn();
    }
}
