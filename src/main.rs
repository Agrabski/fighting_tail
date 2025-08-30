// A complete, self-contained Bevy system for pseudo-turn-based unit movement on a grid.
// This example now organizes the logic into a dedicated plugin for better code structure.
// This code is compatible with Bevy 0.16.1.
mod camera;
mod map;
mod time;
mod game_actions;
mod movement;
mod units;
use bevy::prelude::*;

use crate::{map::{HexGridPlugin, HexPosition}, movement::{MoveUnitEvent, MovementPlugin}, time::GameTimePlugin, units::{AtomicUnitBundle, Echelon}};

/// A resource to track the timer for each turn.
#[derive(Resource)]
pub struct TurnTimer {
    pub timer: Timer,
}

fn setup(mut commands: Commands, mut move_event: EventWriter<MoveUnitEvent>) {
    // Spawn a unit at (0, 0)
    let unit = commands
        .spawn( AtomicUnitBundle::new("Infantry".to_string(), HexPosition::new(0, 0)))
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
        .add_plugins(MovementPlugin)
        .add_plugins(GameTimePlugin)
        .add_plugins(HexGridPlugin)
        .add_plugins(camera::CameraPlugin)
        // Add setup system
        .add_systems(Startup, setup)
        .run();
}
