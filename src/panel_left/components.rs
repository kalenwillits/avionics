use bevy::prelude::*;

#[derive(Component)]
pub struct PanelLeft;

#[derive(Component)]
pub struct TachometerValue;

#[derive(Component)]
pub struct TachometerNeedle;

#[derive(Component, Copy, Clone)]
pub struct EngineOne {
    pub rpm_min: i32,
    pub rpm_max: i32,
    pub normal_operating_min: i32,
    pub normal_operating_max: i32,
}
