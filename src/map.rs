use bevy::{
    prelude::*,
    app::{App, Plugin, Startup},
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    image::{TextureAtlas, TextureAtlasLayout},
    math::uvec2,
    platform::collections::HashMap,
    sprite::Sprite,
    transform::components::Transform, utils::default,
};
use hexx::{shapes, *};

const SPRITE_SIZE: Vec2 = Vec2::new(24.0, 28.0);

pub type HexPosition = Hex;

#[derive(Debug, Resource)]
struct HexGrid {
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
}

fn setup_grid(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("kenney/hexagonTerrain_sheet.png");
    let atlas_layout =
        TextureAtlasLayout::from_grid(uvec2(120, 140), 7, 6, Some(uvec2(2, 2)), None);
    let atlas_layout = atlas_layouts.add(atlas_layout);
    let layout = HexLayout::new(HexOrientation::Pointy).with_rect_size(SPRITE_SIZE);
    let entities = shapes::pointy_rectangle([-14, 14, -16, 16])
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let index = i % (7 * 6);
            let entity = commands
                .spawn((
                    Sprite {
                        custom_size: Some(SPRITE_SIZE),
                        image: texture.clone(),
                        texture_atlas: Some(TextureAtlas {
                            index,
                            layout: atlas_layout.clone(),
                        }),
                        ..default()
                    },
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ))
                .id();
            (coord, entity)
        })
        .collect();
    commands.insert_resource(HexGrid { entities, layout });
}

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid);
    }
}
