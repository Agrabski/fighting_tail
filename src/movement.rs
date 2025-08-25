use std::collections::HashMap;

use bevy::{ecs::component::Component, reflect::Reflect};

pub type Kph = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum MovementMode {
    Tactical,
    Strategic,
}

#[derive(Debug, Clone, Reflect, Component)]
pub struct MovementConfig {
    pub mode: MovementMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DifficultTerrain{

}

#[derive(Debug, Clone, Reflect, Default)]
pub struct MovementStats {
    pub tactical_speed: Kph,
    pub strategic_speed: Kph,
    pub difficult_terrain_penalty: HashMap<DifficultTerrain, f32>,
}

impl MovementStats {
    pub fn get_penalty(&self, terrain: &DifficultTerrain) -> f32 {
        *self.difficult_terrain_penalty.get(terrain).unwrap_or(&1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum MovementPenaltyReason {
    Terrain(DifficultTerrain),
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect, Component)]
pub struct MovemenetPenalty {
    pub value: f32,
    pub reason: MovementPenaltyReason,
}
