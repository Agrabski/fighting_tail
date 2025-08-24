//! A Bevy plugin for displaying and managing a hexagonal map.
//! This plugin is a simple foundation, demonstrating how to handle
//! hex coordinates, spawn entities for each hex, and manage interaction.

use bevy::prelude::*;
use std::collections::HashMap;

// --- Components and Resources ---

/// A component that identifies an entity as a hex tile and stores its axial coordinates.
/// Axial coordinates (q, r) are a common and convenient system for flat-topped hex grids.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Hex {
    pub q: i32,
    pub r: i32,
}

/// A resource to manage the hex map's state, including its size and a mapping
/// from hex coordinates to the corresponding Bevy entity.
#[derive(Resource)]
pub struct HexMap {
    pub size: i32,
    pub hexes: HashMap<Hex, Entity>,
}
#[derive(Resource, Default)]
pub struct HexCoordinates;

// --- Plugin Definition ---

/// The core plugin for the hex map functionality.
pub struct HexMapPlugin;

impl Plugin for HexMapPlugin {
    fn build(&self, app: &mut App) {
        // Add the necessary systems to the app.
        app
            .init_resource::<HexCoordinates>()
            .add_systems(Startup, setup_hex_map)
            ;
    }
}

// --- Systems ---

/// A startup system that creates the hex map and spawns all the hex entities.
/// It uses a simple axial coordinate system to lay out the hexes.
fn setup_hex_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Define the dimensions of the hex grid.
    let map_size = 5; // The radius of the hex grid, including the center.
    let hex_size = Vec2::new(100.0, 100.0);
    
    // Create the HexMap resource and populate it.
    let mut hex_map = HexMap {
        size: map_size,
        hexes: HashMap::new(),
    };

    // Load a simple image asset to use for the hex sprites.
    let hex_texture = asset_server.load("hex.png");
    
    // Create the hexes in a hexagonal shape.
    for q in -map_size..=map_size {
        let r1 = i32::max(-map_size, -q - map_size);
        let r2 = i32::min(map_size, -q + map_size);
        for r in r1..=r2 {
            let s = -q - r;
            // Calculate the position of the hex based on its axial coordinates.
            let x = hex_size.x * (3.0_f32.sqrt() / 2.0 * q as f32 + 0.0 * s as f32);
            let y = hex_size.y * (0.0 * q as f32 + 1.0 * s as f32);

            // Spawn a new entity for this hex tile.
            let hex_entity = commands.spawn((
                Sprite {
                    image: hex_texture.clone(),
                        custom_size: Some(hex_size),
                    ..Default::default()
                },
                Hex { q, r },
                 Transform::from_xyz(x, y, 0.0)
            )).id();
            
            // Add the new hex entity to our map resource.
            hex_map.hexes.insert(Hex { q, r }, hex_entity);
        }
    }
    
    // Insert the newly created map into the Bevy world.
    commands.insert_resource(hex_map);
    
    // Note: The above coordinate conversion is for pointy-top hexes.
    // For flat-top hexes, a more common alternative is:
    // let x = hex_size.x * (3.0_f32.sqrt() * q as f32 + 3.0_f32.sqrt() / 2.0 * r as f32);
    // let y = hex_size.y * (3.0 / 2.0 * r as f32);
    // You may need to adjust the sprite's rotation or the math here.
    // For simplicity, the first method is used above.
    
}


