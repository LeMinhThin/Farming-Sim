use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;

#[derive(Event)]
pub struct TillSoilEvent(pub TilePos);
