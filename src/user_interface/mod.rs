mod hud;
mod theme;
mod unit_list;
use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        schedule::IntoScheduleConfigs,
        system::{Query, Res, Single},
    },
    input::{common_conditions::input_just_pressed, mouse::MouseButton},
    log::debug,
    math::Vec2,
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::{
    map::HexGrid,
    unit_managment::{orders::MoveOrderIssuedEvent, SelectUnitEvent},
    units::Unit,
    user_interface::{hud::HudPlugin, theme::ThemePlugin, unit_list::UnitListPlugin},
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                mouse_left_click.run_if(input_just_pressed(MouseButton::Left)),
                mouse_right_click.run_if(input_just_pressed(MouseButton::Right)),
            ),
        )
        .add_plugins((UnitListPlugin, ThemePlugin, HudPlugin));
    }
}

fn mouse_right_click(
    window: Single<&Window, With<PrimaryWindow>>,
    mut writer: EventWriter<MoveOrderIssuedEvent>,
    map: Res<HexGrid>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single().unwrap();
    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok())
    {
        writer.write(MoveOrderIssuedEvent {
            destination: map.to_hex_coordinates(cursor_pos),
        });
    }
}

fn mouse_left_click(
    window: Single<&Window, With<PrimaryWindow>>,
    mut writer: EventWriter<SelectUnitEvent>,
    camera: Query<(&Camera, &GlobalTransform)>,
    units: Query<(Entity, &Transform), With<Unit>>,
) {
    let (camera, camera_transform) = camera.single().unwrap();
    if let Some(cursor_pos) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok())
    {
        for (entity, transform) in units.iter() {
            let distance =
                cursor_pos.distance(Vec2::new(transform.translation.x, transform.translation.y));
            if distance < 30.0 {
                writer.write(SelectUnitEvent { unit: entity });
                debug!(name: "unit_management:selection", "Selected unit: {:?}", entity);
                break;
            }
        }
    }
}
