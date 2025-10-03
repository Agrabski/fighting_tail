use bevy::{
    asset::AssetServer,
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
    prelude::*,
    reflect::Reflect,
};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, HtmlNode};

use crate::{camera::CameraSetup, unit_managment::SelectUnitEvent};

pub struct UnitListPlugin;

impl Plugin for UnitListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_unit_list.after(CameraSetup))
            .add_systems(Update, on_unit_selected);
    }
}

fn setup_unit_list(
    server: Res<AssetServer>,
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    mut commands: Commands,
) {
    html_comps.register_with_spawn_fn(
        "list_elements_slot",
        server.load("ui/templates/hud/unit_list/list_elements_slot.html"),
        |mut entity_commands| {
            entity_commands.insert(UnitListSlotMarker);
        },
    );
    html_comps.register_with_spawn_fn(
        "unit_list",
        server.load("ui/templates/hud/unit_list/unit_list.html"),
        |mut c| {
            c.insert(UnitListComponent);
        },
    );
}

fn on_unit_selected(
    mut events: EventReader<SelectUnitEvent>,
    list_ui: Single<Entity, With<UnitListSlotMarker>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for event in events.read() {
        // Add a new element to the list for the selected unit
        commands.entity(*list_ui).with_children(|parent| {
            parent.spawn((
                HtmlNode(server.load("ui/templates/hud/unit_list/unit_list_element.html")),
                UnitListElementComponent { unit: event.unit },
            ));
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
struct UnitListComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
struct UnitListElementComponent {
    pub unit: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
struct UnitListSlotMarker;
