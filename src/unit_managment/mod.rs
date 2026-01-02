pub mod orders;
pub mod selection;
use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity, event::{Event, EventReader}, message::{Message, MessageReader}, resource::Resource, schedule::{IntoScheduleConfigs, common_conditions}, system::ResMut
    },
    reflect::Reflect,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::unit_managment::{orders::OrdersPlugin, selection::SelectionPlugin};

pub struct UnitManagementPlugin;

impl Plugin for UnitManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrdersPlugin)
            .add_plugins(SelectionPlugin)
            .add_event::<SelectUnitMessage>()
            .register_type::<SelectUnitMessage>()
            .insert_resource(SelectedUnitList::default())
            .register_type::<SelectedUnitList>()
            .add_systems(
                Update,
                add_units_to_selection.run_if(common_conditions::on_event::<SelectUnitMessage>),
            );
    }
}

#[derive(Debug, Reflect, Message)]
pub struct SelectUnitMessage {
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

#[derive(Resource, Reflect, Default, Debug)]
pub struct SelectedUnitList {
    selected_units: Vec<Entity>,
}
