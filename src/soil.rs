use crate::structs::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use std::collections::HashMap;

const TOP_LEFT: u32 = 0;
const TOP: u32 = 1;
const TOP_RIGHT: u32 = 2;
const LEFT: u32 = 12;
const CENTER: u32 = 13;
const RIGHT: u32 = 14;
const BOTTOM_LEFT: u32 = 24;
const BOTTOM: u32 = 25;
const BOTTOM_RIGHT: u32 = 26;

const NORTH: u32 = 3;
const NORTH_SOUTH: u32 = 15;
const SOUTH: u32 = 27;
const DOT: u32 = 39;
const EAST: u32 = 38;
const EAST_WEST: u32 = 37;
const WEST: u32 = 36;
const INTERSECTION: u32 = 56;

// Looks horrible to me
fn to_u8(neighbors: &Neighbors<TilePos>) -> u8 {
    let mut output: u8 = 0;
    output |= (neighbors.north_west.is_some() as u8) << 7;
    output |= (neighbors.north.is_some() as u8) << 6;
    output |= (neighbors.north_east.is_some() as u8) << 5;
    output |= (neighbors.west.is_some() as u8) << 4;
    output |= (neighbors.east.is_some() as u8) << 3;
    output |= (neighbors.south_west.is_some() as u8) << 2;
    output |= (neighbors.south.is_some() as u8) << 1;
    output |= neighbors.south_east.is_some() as u8;
    output
}

// Don't ask
fn init_connected_tile_data(mut tile_data: ResMut<ConnectedTileData>) {
    let mut map: HashMap<u8, u32> = HashMap::new();
    map.insert(0b00000000u8, DOT);
    map.insert(0b00001011u8, TOP_LEFT);
    map.insert(0b00011111u8, TOP);
    map.insert(0b00010110u8, TOP_RIGHT);
    map.insert(0b01101011u8, LEFT);
    map.insert(0b11010110u8, RIGHT);
    map.insert(0b01101000u8, BOTTOM_LEFT);
    map.insert(0b11111000u8, BOTTOM);
    map.insert(0b11010000u8, BOTTOM_RIGHT);
    map.insert(0b00000000u8, CENTER);
    map.insert(0b01000000u8, NORTH);
    map.insert(0b01000010u8, NORTH_SOUTH);
    map.insert(0b01000000u8, SOUTH);
    map.insert(0b00010000u8, EAST);
    map.insert(0b00011000u8, EAST_WEST);
    map.insert(0b00001000u8, WEST);
    map.insert(0b01011010u8, INTERSECTION);

    tile_data.0 = map;
}

fn on_till(
    mut commands: Commands,
    mut till_event: EventReader<TillSoilEvent>,
    mut soil_layer: Query<(&mut TileStorage, &TilemapSize), With<SoilLayer>>,
    texture_map: Res<ConnectedTileData>
) {
    let (soil_layer, map_size) = soil_layer.single_mut();

    for event in till_event.read() {
        change_soil_texture((&soil_layer, map_size), &mut commands, &event.0, &texture_map.0)
    }
}

// Oh my god recursion, here we go
fn change_soil_texture(tilemap: (&TileStorage, &TilemapSize), commands:&mut Commands, pos: &TilePos, map: &HashMap<u8, u32>) {
    let (storage, size) = tilemap;
    let neighbors = Neighbors::get_square_neighboring_positions(pos, size, true);
    let key = to_u8(&neighbors);
    let index: u32;
    if let Some(i) = map.get(&key) {
        index = *i
    } else {
        index = CENTER;
    }
    commands.entity(storage.get(pos).expect(&format!("Can't access tile index {:?}", pos))).insert(TileTextureIndex(index));
}

pub struct SoilPlugin;
impl Plugin for SoilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_connected_tile_data)
            .add_systems(Update, on_till);
    }
}
