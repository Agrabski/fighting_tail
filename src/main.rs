// A complete, self-contained Bevy system for pseudo-turn-based unit movement on a grid.
// This example now organizes the logic into a dedicated plugin for better code structure.
// This code is compatible with Bevy 0.16.1.
mod camera;
mod game_actions;
mod map;
mod movement;
mod time;
mod units;
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::WorldInspectorPlugin,
};

use crate::{
    map::{HexGridPlugin, HexPosition},
    movement::{MoveUnitEvent, MovementPlugin},
    time::GameTimePlugin,
    units::AtomicUnitBundle,
};

fn setup(mut commands: Commands, mut move_event: EventWriter<MoveUnitEvent>) {
    // Spawn a unit at (0, 0)
    let unit = commands
        .spawn(AtomicUnitBundle::new(
            "Infantry".to_string(),
            HexPosition::new(0, 0),
        ))
        .id();

    // Send a movement event to the unit to move to (10, 5)
    move_event.write(MoveUnitEvent {
        unit,
        destination: HexPosition { x: 10, y: 5 },
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MovementPlugin)
        .add_plugins(GameTimePlugin)
        .add_plugins(HexGridPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}
