use std::ops::{Deref, DerefMut};

use bevy::app::App;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use leafwing_input_manager::prelude::*;

use crate::game_actions::GameActions;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub struct CameraSetup;

pub struct CameraPlugin;
//todo: camera movement should be independent of game logic clock

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraSettings>();
        app.insert_resource(CameraSettings {
            movement_speed: 300.0,
            zoom_speed: 10.0,
        });

        app.add_plugins(InputManagerPlugin::<GameActions>::default());
        app.add_systems(Startup, setup_camera.in_set(CameraSetup));
        app.add_systems(Update, update_camera);
    }
}

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct CameraSettings {
    pub movement_speed: f32,
    pub zoom_speed: f32,
}

const MIN_ZOOM: f32 = 0.05;
const MAX_ZOOM: f32 = 50.0;
const DEFAULT_CAMERA_ZOOM: f32 = 0.05;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Camera"),
        Camera::default(),
        Camera2d,
        Transform::from_translation(Vec3::Z * 1000.0),
        RenderLayers::from_layers(&[0, 1]),
        Projection::Orthographic(OrthographicProjection::default_2d()),
        InputMap::new([
            (GameActions::MoveUp, KeyCode::KeyW),
            (GameActions::MoveDown, KeyCode::KeyS),
            (GameActions::MoveLeft, KeyCode::KeyA),
            (GameActions::MoveRight, KeyCode::KeyD),
            (GameActions::ZoomIn, KeyCode::KeyQ),
            (GameActions::ZoomOut, KeyCode::KeyE),
        ])
        .with(GameActions::Select, MouseButton::Left),
    ));
}

fn update_camera(
    mut query: Single<(&mut Transform, &mut Projection, &ActionState<GameActions>), With<Camera2d>>,
    settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    let (transform, projection, input) = query.deref_mut();
    let delta = time.delta_secs();

    if let Projection::Orthographic(projection) = projection.deref_mut() {
        let zoom = projection.scale.ln() + (get_zoom(input) * settings.zoom_speed * delta);
        projection.scale = zoom.exp().clamp(MIN_ZOOM, MAX_ZOOM);
        if projection.scale == 0.0 {
            projection.scale = DEFAULT_CAMERA_ZOOM;
        }
        let movement = get_movement(input) * (settings.movement_speed * delta * projection.scale);
        transform.translation += movement;
    }
}

fn get_movement(input: &ActionState<GameActions>) -> Vec3 {
    let mut movement = Vec3::default();

    if input.pressed(&GameActions::MoveUp) {
        movement.y += 1.0;
    }
    if input.pressed(&GameActions::MoveDown) {
        movement.y -= 1.0;
    }
    if input.pressed(&GameActions::MoveLeft) {
        movement.x -= 1.0;
    }
    if input.pressed(&GameActions::MoveRight) {
        movement.x += 1.0;
    }

    movement
}

fn get_zoom(input: &ActionState<GameActions>) -> f32 {
    let mut zoom = 0.0;

    if input.pressed(&GameActions::ZoomIn) {
        zoom -= 1.0;
    }
    if input.pressed(&GameActions::ZoomOut) {
        zoom += 1.0;
    }

    zoom
}
