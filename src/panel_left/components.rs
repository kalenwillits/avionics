use bevy::prelude::*;

#[derive(Component)]
pub struct PanelLeft;

#[derive(Component)]
pub struct TachometerValue;

#[derive(Component)]
pub struct TachometerNeedle;

#[derive(Component, Copy, Clone)]
pub struct EngineOne {
    pub min_rpm: i64,
    pub max_rpm: i64,
}
