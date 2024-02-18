use crate::structs::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::*;
use bevy_ecs_tilemap::helpers::square_grid::SquarePos;
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

fn on_till(mut till_event: EventReader<TillSoilEvent>, mut soil_layer: ResMut<SoilMap>) {
    for event in till_event.read() {
        let (x, y) = (event.0.x as usize, event.0.y as usize);
        if x > soil_layer.0[0].len() {
            warn!("till coord X out of bound");
            return;
        }
        if y > soil_layer.0.len() {
            warn!("till coord Y out of bound");
            return;
        }
        soil_layer.0[y][x] = SoilState::Tilled;
    }
}

fn write_to_layer(
    mut till_event: EventReader<TillSoilEvent>,
    mut soil_layer: Query<&mut TileStorage, With<SoilLayer>>,
    mut commands: Commands,
    soil_map: Res<SoilMap>,
    texture_map: Res<ConnectedTileData>,
) {
    let soil_layer = soil_layer.single_mut();

    for _ in till_event.read() {
        for row in 0..soil_map.0.len() {
            for cell in 0..soil_map.0[0].len() {
                let pos = TilePos::new(cell as u32, row as u32);
                let neighbors = get_neighbors(&soil_map.0, &pos);
                let key = neighbors_to_u8(neighbors, SoilState::Tilled);

                if let Some(index) = texture_map.0.get(&key) {
                    commands.entity(soil_layer.get(&pos).unwrap()).insert(TileTextureIndex(*index));
                } else {
                    commands.entity(soil_layer.get(&pos).unwrap()).insert(TileTextureIndex(CENTER));
                }
            }
        }
    }
}

/*
// Oh my god recursion, here we go
fn change_soil_texture(
    tilemap: (&TileStorage, &TilemapSize),
    commands: &mut Commands,
    pos: &TilePos,
    map: &HashMap<u8, u32>,
) {
    let (storage, size) = tilemap;
    let neighbors = Neighbors::get_square_neighboring_positions(pos, size, true);
    let key = to_u8(&neighbors);
    let index: u32;
    if let Some(i) = map.get(&key) {
        index = *i
    } else {
        index = CENTER;
    }
    commands
        .entity(
            storage
                .get(pos)
                .expect(&format!("Can't access tile index {:?}", pos)),
        )
        .insert(TileTextureIndex(index));
}
*/

pub struct SoilPlugin;
impl Plugin for SoilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_connected_tile_data)
            .add_systems(Update, (on_till, write_to_layer).chain());
    }
}

fn get_neighbors(vec: &Vec<Vec<SoilState>>, cell: &TilePos) -> Vec<Option<SoilState>> {
    let size = TilemapSize::new(vec[0].len() as u32, vec.len() as u32);
    let square_pos = SquarePos::from(cell);
    let func = |direction: SquareDirection| square_pos.offset(&direction).as_tile_pos(&size);

    let neighbors = Neighbors::from_directional_closure(func);
    let neighbors = neighbors_to_array(&neighbors);
    neighbors
        .into_iter()
        .map(|elem| {
            if let Some(index) = elem {
                Some(vec[index.y as usize][index.x as usize])
            } else {
                None
            }
        })
        .collect()
}

fn neighbors_to_u8(neighbors: Vec<Option<SoilState>>, match_cond: SoilState) -> u8 {
    let mut ret = u8::MIN;
    let mut counter = 0;
    for i in 7..=0 {
        if Some(match_cond) == neighbors[counter] {
            ret |= 1 << i
        }
        counter += 1;
    }
    ret
}

const fn neighbors_to_array(neighbors: &Neighbors<TilePos>) -> [Option<TilePos>; 8] {
    [
        neighbors.north_west,
        neighbors.north,
        neighbors.north_east,
        neighbors.west,
        neighbors.east,
        neighbors.south_west,
        neighbors.south,
        neighbors.south_east,
    ]
}
