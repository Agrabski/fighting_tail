use bevy::{
    asset::AssetServer,
    ecs::{
        schedule::{IntoScheduleConfigs, common_conditions},
        system::{Commands, Res},
    },
    prelude::*,
    reflect::Reflect,
};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, HtmlNode, TemplateProperties};

use crate::{
    camera::CameraSetup,
    unit_managment::SelectUnitMessage,
    units::{self, Unit},
};

pub struct UnitListPlugin;

impl Plugin for UnitListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_unit_list.after(CameraSetup))
            .add_systems(Update, (on_unit_selected, setup_unit_list_element));
    }
}

const SELECT_UNIT_ELEMENT_FN: &str = "user_interface::unit_list::select_unit";

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

    html_funcs.register(
        SELECT_UNIT_ELEMENT_FN,
        |input: In<Entity>, mut cmd: Commands| {
            /*
            cmd.entity(input.entity()).add(|cmd| {
                if let Some(selected) = units::get_selected_unit() {
                    cmd.insert(SelectUnitEvent { unit: selected });
                }
            }); */
        },
    );
}

fn on_unit_selected(
    mut events: EventReader<SelectUnitMessage>,
    list_ui: Single<Entity, With<UnitListSlotMarker>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for event in events.read() {
        commands.entity(*list_ui).with_children(|parent| {
            parent.spawn((
                HtmlNode(server.load("ui/templates/hud/unit_list/unit_list_element.html")),
                UnitListElementComponent { unit: event.unit },
                TemplateProperties::default().with("action", SELECT_UNIT_ELEMENT_FN),
            ));
        });
    }
}

fn setup_unit_list_element(
    mut q: Query<
        (&mut TemplateProperties, &UnitListElementComponent),
        Added<UnitListElementComponent>,
    >,
    units: Query<&Name, With<Unit>>,
) {
    for (mut props, element) in q.iter_mut() {
        if let Ok(name) = units.get(element.unit) {
            props.set("unit_name", &name.to_string());
        } else {
            props.set("unit_name", "Unknown");
        }
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
