pub mod orders;
pub mod selection;
use std::ops::Index;

use bevy::ecs::{
    schedule::{IntoScheduleConfigs, common_conditions},
    system::ResMut,
};
use bevy::prelude::*;

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
                (
                    add_units_to_selection
                        .run_if(common_conditions::on_message::<SelectUnitMessage>),
                    remove_units_from_selection
                        .run_if(common_conditions::on_message::<DeselectUnitMessage>),
                ),
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
        if let Some(index) = selected_units
            .selected_units
            .iter()
            .position(|e| *e == event.unit)
        {
            selected_units.selected_units.remove(index);
        }
    }
}

#[derive(Resource, Reflect, Default, Debug)]
pub struct SelectedUnitList {
    selected_units: Vec<Entity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn select_adds_unit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(UnitManagementPlugin);

        // spawn a unit entity (no components required for selection by message)
        let entity = app.world_mut().spawn_empty().id();

        // write a select message in startup so it will be read during the first Update
        app.add_systems(
            Startup,
            move |mut writer: bevy::ecs::message::MessageWriter<SelectUnitMessage>| {
                writer.write(SelectUnitMessage { unit: entity });
            },
        );

        app.update(); // runs Update stage systems including add_units_to_selection

        let selected = app.world().resource::<SelectedUnitList>();
        assert!(selected.selected_units.contains(&entity));
        // ensure duplicates are not introduced
        assert_eq!(
            selected
                .selected_units
                .iter()
                .filter(|e| **e == entity)
                .count(),
            1
        );
    }

    #[test]
    fn deselect_removes_unit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(UnitManagementPlugin);

        let entity = app.world_mut().spawn_empty().id();

        // Stage 1: select the unit
        app.add_systems(
            Startup,
            move |mut writer: bevy::ecs::message::MessageWriter<SelectUnitMessage>| {
                writer.write(SelectUnitMessage { unit: entity });
            },
        );

        app.update();

        {
            let selected = app.world().resource::<SelectedUnitList>();
            assert!(selected.selected_units.contains(&entity));
        }

        // Stage 2: write a deselect message in Update *after* add_units_to_selection
        app.add_systems(
            Update,
            (move |mut writer: bevy::ecs::message::MessageWriter<DeselectUnitMessage>| {
                writer.write(DeselectUnitMessage { unit: entity });
            })
            .after(add_units_to_selection),
        );

        // run two updates: first to dispatch the write, second to let the selection systems process the message
        app.update();
        app.update();

        let selected = app.world().resource::<SelectedUnitList>();
        assert!(!selected.selected_units.contains(&entity));
    }
}
