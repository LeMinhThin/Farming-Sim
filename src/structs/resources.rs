use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct SelectedCell(pub Option<Vec2>);

#[derive(Resource, Default)]
pub struct ConnectedTileData(pub HashMap<u8, u32>);
