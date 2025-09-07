use bevy::{ecs::resource::Resource, reflect::Reflect};

pub struct ResourcesPlugin;
impl bevy::app::Plugin for ResourcesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ResourceType>()
            .register_type::<ResourceStack>()
            .register_type::<ResourceAmout>()
            .register_type::<ResourceTypes>()
            .init_resource::<ResourceTypes>()
            .insert_resource::<PalletizationConfig>(PalletizationConfig {
                pallete_volume: 50.0,
            });
    }
}

#[derive(Debug, Reflect, Resource)]
pub struct PalletizationConfig {
    pub pallete_volume: VolumeInLitters,
}

pub type ResourceTypeId = u32;
pub type VolumeInLitters = f32;
pub type WeightInKilograms = f32;

#[derive(Debug, Reflect, Default)]
pub struct ResourceType {
    pub name: String,
    pub unit_weight: WeightInKilograms,
    pub unit_volume: VolumeInLitters,
    pub palletization: PalletizationInfo,
}

#[derive(Debug, Reflect, Default)]
pub enum PalletizationInfo {
    #[default]
    CannotBePalletized,
    Palletized {
        units_per_pallet: u32,
    },
}

#[derive(Reflect, Default, Resource)]
pub struct ResourceTypes {
    pub types: Vec<ResourceType>,
}

#[derive(Debug, Reflect)]
pub struct ResourceStack {
    pub resource_type: ResourceTypeId,
    pub amount: ResourceAmout,
}

#[derive(Debug, Reflect)]
pub enum ResourceAmout {
    Fluid { count: f32 },
    Pallets { count: u32 },
}
