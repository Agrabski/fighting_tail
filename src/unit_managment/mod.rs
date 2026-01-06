pub mod orders;
pub mod selection;
use std::ops::Index;

use bevy::prelude::*;
use bevy::{
    ecs::{
        schedule::{IntoScheduleConfigs, common_conditions},
        system::ResMut,
    },
};

use crate::unit_managment::{orders::OrdersPlugin, selection::SelectionPlugin};

pub struct UnitManagementPlugin;

impl Plugin for UnitManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrdersPlugin)
            .add_plugins(SelectionPlugin)
            .add_message::<SelectUnitMessage>()
            .add_message::<DeselectUnitMessage>()
            .register_type::<SelectUnitMessage>()
            .insert_resource(SelectedUnitList::default())
            .register_type::<SelectedUnitList>()
            .add_systems(
                Update,
                add_units_to_selection.run_if(common_conditions::on_message::<SelectUnitMessage>),
            );
    }
}

#[derive(Debug, Reflect, Message)]
pub struct SelectUnitMessage {
    pub unit: Entity,
}

#[derive(Debug, Reflect, Message)]
pub struct DeselectUnitMessage {
    pub unit: Entity,
}

fn add_units_to_selection(
    mut selected_units: ResMut<SelectedUnitList>,
    mut select_unit_events: MessageReader<SelectUnitMessage>,
) {
    for event in select_unit_events.read() {
        if !selected_units.selected_units.contains(&event.unit) {
            selected_units.selected_units.push(event.unit);
        }
    }
}


fn remove_units_from_selection(
    mut selected_units: ResMut<SelectedUnitList>,
    mut select_unit_events: MessageReader<DeselectUnitMessage>,
) {
    for event in select_unit_events.read() {
        if let Some(index)= selected_units.selected_units.iter().position(|e| *e==event.unit) {
            selected_units.selected_units.remove(index);
        }
    }
}

#[derive(Resource, Reflect, Default, Debug)]
pub struct SelectedUnitList {
    selected_units: Vec<Entity>,
}
