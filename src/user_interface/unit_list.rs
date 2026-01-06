use bevy::{
    asset::AssetServer,
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Res},
    },
    log,
    prelude::*,
};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, HtmlNode, Tags, TemplateProperties};

use crate::{
    camera::CameraSetup,
    unit_managment::{DeselectUnitMessage, SelectUnitMessage},
    units::Unit,
};

pub struct UnitListPlugin;

impl Plugin for UnitListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_unit_list.after(CameraSetup))
            .add_systems(
                Update,
                (
                    on_unit_selected,
                    on_unit_deselected,
                    setup_unit_list_element,
                ),
            );
    }
}

const DESELECT_UNIT_ELEMENT_FN: &str = "user_interface::unit_list::deselect_unit";

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
        DESELECT_UNIT_ELEMENT_FN,
        |In(input): In<Entity>,
        tags: Query<&Tags>,
         mut cmd: MessageWriter<DeselectUnitMessage>| {
            if let Some(tags) = tags.get(input.entity()).ok() {
                let unit = tags.get("unit_id").unwrap();
                cmd.write(DeselectUnitMessage {
                    unit: Entity::from_bits(unit.parse().unwrap())
                });
            } else {
                log::warn!(
                    "Tried to deselect unit from unit list, but no UnitListElementComponent found on entity {:?}",
                    input.entity()
                );
            }
        },
    );
}

fn on_unit_selected(
    mut events: MessageReader<SelectUnitMessage>,
    list_ui: Single<Entity, With<UnitListSlotMarker>>,
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    for event in events.read() {
        commands.entity(*list_ui).with_children(|parent| {
            parent.spawn((
                HtmlNode(server.load("ui/templates/hud/unit_list/unit_list_element.html")),
                UnitListElementComponent { unit: event.unit },
                TemplateProperties::default()
                    .with("action", DESELECT_UNIT_ELEMENT_FN)
                    .with("unit_id", &event.unit.to_bits().to_string()),
            ));
        });
    }
}

fn on_unit_deselected(
    mut events: MessageReader<DeselectUnitMessage>,
    q: Query<(Entity, &UnitListElementComponent)>,
    mut commands: Commands,
) {
    for event in events.read() {
        log::info!("Deselecting unit from UI: {:?}", event.unit);
        for (ent, element) in q.iter() {
            if element.unit == event.unit {
                commands.entity(ent).despawn();
            }
        }
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
