mod path_finding;
use std::collections::HashMap;

use bevy::{ecs::{component::Component, entity::Entity, event::Event}, reflect::Reflect};

use crate::{map::HexPosition, movement::path_finding::PathFindingPlugin};

pub type Kph = f32;

pub struct MovementPlugin;

impl bevy::app::Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<MovementMode>()
            .register_type::<MovementConfig>()
            .register_type::<DifficultTerrain>()
            .register_type::<MovementStats>()
            .register_type::<MovementPenaltyReason>()
            .register_type::<MovemenetPenalty>()
            .register_type::<Path>()
            .register_type::<GamePosition>()
            .register_type::<MovingTowards>()
            .add_event::<MoveUnitEvent>()
            .add_plugins(PathFindingPlugin);
    }
}

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

#[derive(Component, Debug, Reflect)]
pub struct Path {
    pub waypoints: Vec<HexPosition>,
}

#[derive(Component, Debug, Reflect)]
pub struct GamePosition {
    pub hex: HexPosition,
}


/// Event to trigger a unit's movement to a new destination.
#[derive(Event)]
pub struct MoveUnitEvent {
    pub unit: Entity,
    pub destination: HexPosition,
}

pub const PROGRESS_ZERO: f32 = 0.0;
pub const PROGRESS_COMPLETE: f32 = 100.0;
#[derive(Component, Debug, Reflect)]
pub struct MovingTowards {
    pub destination: HexPosition,
    pub progress: f32, 
}

impl MovingTowards {
    pub fn new(destination: HexPosition) -> Self {
        Self {
            destination,
            progress: PROGRESS_ZERO,
        }
    }
    
}
