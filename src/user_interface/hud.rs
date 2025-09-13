use bevy::prelude::*;
use bevy_hui::prelude::*;

use crate::camera::CameraSetup;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup.after(CameraSetup));
    }
}

fn startup(
    server: Res<AssetServer>,
    mut html_comps: HtmlComponents,
    mut html_funcs: HtmlFunctions,
    mut commands: Commands,
) {
    // advanced register, with spawn functions
    html_comps.register_with_spawn_fn(
        "hud",
        server.load("ui/templates/hud/index.html"),
        |mut entity_commands| {},
    );

    commands.spawn(HtmlNode(server.load("ui/templates/hud/index.html")));
}
