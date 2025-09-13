use bevy::{
    app::{App, Plugin, Startup},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        schedule::IntoScheduleConfigs,
        system::{Commands, In, Res},
    },
    log::debug,
};
use bevy_hui::prelude::{HtmlComponents, HtmlFunctions, HtmlNode};

use crate::{GameState, camera::CameraSetup, user_interface::theme::Theme};

pub struct UnitListPlugin;

impl Plugin for UnitListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_unit_list.after(CameraSetup));
    }
}

fn setup_unit_list(
    server: Res<AssetServer>,
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    mut commands: Commands,
) {
    // advanced register, with spawn functions
    html_comps.register_with_spawn_fn(
        "unit_list",
        server.load("ui/templates/hud/unit_list.html"),
        |mut entity_commands| {},
    );
}
