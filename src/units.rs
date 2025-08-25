use bevy::{ecs::{component::Component, resource::Resource}, platform::collections::HashMap, reflect::Reflect};

use crate::movement::MovementStats;

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
}


pub enum Echelon {
    Squad,
    Platoon,
    Company,
    Battalion,
    Brigade,
}
