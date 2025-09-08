mod camera;
mod game_actions;
mod map;
mod movement;
mod resources;
mod time;
mod unit_managment;
mod units;
mod user_interface;
use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_lunex::UiLunexPlugins;

use crate::{
    map::{HexGridPlugin, HexPosition},
    movement::{MoveUnitEvent, MovementPlugin},
    resources::ResourcesPlugin,
    time::GameTimePlugin,
    unit_managment::UnitManagementPlugin,
    units::{AtomicUnitBundle, UnitPlugin},
    user_interface::UserInterfacePlugin,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Running,
    Paused,
}

fn setup(mut commands: Commands, mut move_event: EventWriter<MoveUnitEvent>) {
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
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,naga=warn,bevy_render=warn,bevy_ecs=info,pathfinding=debug,movement=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(MovementPlugin)
        .add_plugins(GameTimePlugin)
        .add_plugins(HexGridPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(UnitPlugin)
        .add_plugins(UnitManagementPlugin)
        .add_plugins(UserInterfacePlugin)
        .add_plugins(ResourcesPlugin)
        .add_systems(Startup, setup)
        .add_plugins((UiLunexPlugins))
        .insert_state(GameState::Running)
        .run();
}
