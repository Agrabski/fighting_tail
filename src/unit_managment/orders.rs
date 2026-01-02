use bevy::{
    app::{Plugin, Update},
    ecs::{
        event::{Event, EventReader, EventWriter}, message::{Message, MessageReader, MessageWriter}, schedule::{IntoScheduleConfigs, common_conditions}, system::Res
    },
    reflect::Reflect,
};

use crate::{map::HexPosition, movement::MoveUnitMessage, unit_managment::SelectedUnitList};

pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_message::<MoveOrderIssuedMessage>()
            .register_type::<MoveOrderIssuedMessage>()
            .add_systems(
                Update,
                issue_move_order.run_if(common_conditions::on_message::<MoveOrderIssuedMessage>),
            );
    }
}

#[derive(Debug, Reflect, Message)]
pub struct MoveOrderIssuedMessage {
    pub destination: HexPosition,
}

fn issue_move_order(
    mut orders: MessageReader<MoveOrderIssuedMessage>,
    units: Res<SelectedUnitList>,
    mut unit_orders: MessageWriter<MoveUnitMessage>,
) {
    for order in orders.read() {
        for &unit in &units.selected_units {
            unit_orders.write(MoveUnitMessage {
                unit,
                destination: order.destination,
            });
        }
    }
}
