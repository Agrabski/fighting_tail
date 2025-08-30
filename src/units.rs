use bevy::{
    color::LinearRgba, ecs::{bundle::Bundle, component::Component, resource::Resource}, platform::collections::HashMap, reflect::Reflect, sprite::Sprite, transform::components::Transform
};

use crate::{
    map::HexPosition,
    movement::{GamePosition, MovementStats},
};

pub type UnitTypeId = String;

#[derive(Reflect, Default, Debug)]
pub struct UnitDetails {
    pub movement_stats: MovementStats,
}

#[derive(Resource, Reflect, Default)]
pub struct UnitTypeList {
    pub types: HashMap<UnitTypeId, UnitDetails>,
}

#[derive(Component, Reflect, Debug)]
pub struct Unit {
    pub unit_type: UnitTypeId,
    pub echelon: Echelon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Echelon {
    Squad,
    Platoon,
    Company,
    Battalion,
    Brigade,
}

#[derive(Component)]
pub struct AtomicUnit;

#[derive(Bundle)]
pub struct AtomicUnitBundle {
    unit: Unit,
    atomic: AtomicUnit,
    position: GamePosition,
    sprite: Sprite,
    transform: Transform
}

impl AtomicUnitBundle {
    pub fn new(unit_type: UnitTypeId, position: HexPosition) -> Self {
        Self {
            unit: Unit {
                unit_type,
                echelon: Echelon::Squad,
            },
            atomic: AtomicUnit,
            position: GamePosition { hex: position },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(bevy::prelude::Vec2::new(20.0, 20.0)),
                color: bevy::prelude::Color::LinearRgba(LinearRgba::RED),
                ..Default::default()
            }
        }
    }
}
