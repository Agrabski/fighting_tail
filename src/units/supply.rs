use bevy::{
    app::{App, Plugin},
    ecs::component::Component,
    reflect::Reflect,
};

use crate::resources::{ResourceStack, VolumeInLitters, WeightInKilograms};

pub struct SupplyPlugin;
impl Plugin for SupplyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SupplyStorage>();
    }
}

#[derive(Debug, Reflect, Component)]
pub struct SupplyStorage {
    pub storage: Vec<ResourceStack>,
    pub max_weight: Option<WeightInKilograms>,
    pub max_volume: Option<VolumeInLitters>,
}
