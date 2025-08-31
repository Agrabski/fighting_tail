use bevy::{
    app::{App, Plugin, Startup}, asset::{AssetServer, Assets}, ecs::{
        entity::Entity,
        resource::Resource,
        system::{Commands, Res, ResMut},
    }, image::{TextureAtlas, TextureAtlasLayout}, math::uvec2, platform::collections::HashMap, prelude::*, sprite::Sprite, transform::components::Transform, utils::default
};
use hexx::{algorithms::a_star, shapes, *};

use crate::movement::{GamePosition, MovementConfig, MovingTowards};

const SPRITE_SIZE: Vec2 = Vec2::new(24.0, 28.0);
pub const HEX_RADIUS_IN_METERS: f32 = 100.0;

pub type HexPosition = Hex;

#[derive(Debug, Resource)]
pub struct HexGrid {
    entities: HashMap<Hex, Entity>,
    layout: HexLayout,
}

impl HexGrid {
    pub fn entity_at(&self, hex: Hex) -> Option<Entity> {
        self.entities.get(&hex).cloned()
    }

    pub fn to_global_coordinates(&self, hex:Hex) -> Vec2 {
        self.layout.hex_to_world_pos(hex)
    }

    pub fn find_path(&self, start:Hex, end:Hex, config: &MovementConfig) -> Vec<Hex> {
        a_star(start, end, |a,b| Some( 1)).expect("Pathfinding failed")
    }
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
    let entities = shapes::hexagon(Hex::ZERO, 150)
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

fn sync_tranforms(
    grid: Res<HexGrid>,
    mut query: Query<(&GamePosition, &mut Transform, Option<&MovingTowards>), Changed<GamePosition>>,
) {
    for (game_pos, mut transform, moving) in query.iter_mut() {
        let world_pos = grid.to_global_coordinates(game_pos.hex);
        if let Some(moving) = moving {
            let target_pos = grid.to_global_coordinates(moving.destination);
            let direction = (target_pos - world_pos).normalize_or_zero();
            transform.translation.x += direction.x * 2.0;
            transform.translation.y += direction.y * 2.0;
            if (transform.translation.x - target_pos.x).abs() < 2.0
                && (transform.translation.y - target_pos.y).abs() < 2.0
            {
                transform.translation.x = target_pos.x;
                transform.translation.y = target_pos.y;
            }
            continue;
        }
        transform.translation.x = world_pos.x;
        transform.translation.y = world_pos.y;
    }
}

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid)
        .add_systems(Update, sync_tranforms);
    }
}
