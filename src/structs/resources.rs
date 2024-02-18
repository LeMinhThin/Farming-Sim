use super::SoilState;
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapSize;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct SelectedCell(pub Option<Vec2>);

#[derive(Resource, Default)]
pub struct ConnectedTileData(pub HashMap<u8, u32>);

#[derive(Resource)]
pub struct SoilMap(pub Vec<Vec<SoilState>>);

impl Default for SoilMap {
    fn default() -> Self {
        // TODO unhardcore this somehow
        let size = TilemapSize::new(40, 27);
        let mut row = Vec::with_capacity(size.y as usize);
        let mut column = Vec::with_capacity(size.x as usize);
        column.fill(SoilState::UnTilled);
        row.fill(column);
        
        SoilMap(row)
    }
}
