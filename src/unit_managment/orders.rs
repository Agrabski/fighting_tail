use bevy::{
    app::{Plugin, Update},
    ecs::{
        event::{Event, EventReader, EventWriter},
        schedule::{IntoScheduleConfigs, common_conditions},
        system::Res,
    },
    reflect::Reflect,
};

use crate::{map::HexPosition, movement::MoveUnitEvent, unit_managment::SelectedUnitList};

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<MoveOrderIssuedEvent>()
            .register_type::<MoveOrderIssuedEvent>()
            .add_systems(
                Update,
                issue_move_order.run_if(common_conditions::on_event::<MoveOrderIssuedEvent>),
            );
    }
}

#[derive(Debug, Reflect, Event)]
pub struct MoveOrderIssuedEvent {
    pub destination: HexPosition,
}

fn issue_move_order(
    mut orders: EventReader<MoveOrderIssuedEvent>,
    units: Res<SelectedUnitList>,
    mut unit_orders: EventWriter<MoveUnitEvent>,
) {
    for order in orders.read() {
        for &unit in &units.selected_units {
            unit_orders.write(MoveUnitEvent {
                unit,
                destination: order.destination,
            });
        }
    }
}
