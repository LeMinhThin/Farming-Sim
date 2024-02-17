use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod player;
mod soil;
mod structs;

use player::PlayerPlugin;
use soil::SoilPlugin;
use structs::*;

// Look into the tilesheet for explanation
pub const TILE_SIZE: f32 = 16.;
const EMPTY_TILE: u32 = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SoilPlugin)
        .add_systems(Startup, (setup, spawn_till_layer))
        .add_systems(Update, move_entities)
        .add_systems(Update, animate)
        .add_event::<TillSoilEvent>()
        .init_resource::<SelectedCell>()
        .init_resource::<ConnectedTileData>()
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Gravity::ZERO)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.4;
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 1280.,
        height: 720.,
    };

    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets_server.load("maps/map.ldtk"),
        ..Default::default()
    });
}

fn spawn_till_layer(mut commands: Commands, assets_server: Res<AssetServer>) {
    // TODO unhardcode these
    let map_size = TilemapSize { x: 40, y: 27 };
    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let tilemap_entity = commands.spawn_empty().id();
    let map_type = TilemapType::default();
    let texture = assets_server.load("res/Tilesets/Tilled_Dirt.png");
    let mut tile_storage = TileStorage::empty(map_size);

    bevy_ecs_tilemap::helpers::filling::fill_tilemap(
        TileTextureIndex(EMPTY_TILE),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            map_type,
            tile_size,
            grid_size: tile_size.into(),
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture),
            transform: Transform::from_xyz(TILE_SIZE /2., TILE_SIZE / 2., 10.),
            ..Default::default()
        },
        SoilLayer,
    ));
}

fn move_entities(mut entities: Query<(&Acceleration, &mut LinearVelocity)>, time: Res<Time>) {
    for (accel, mut velocity) in entities.iter_mut() {
        velocity.0 += accel.0 * time.delta_seconds();
    }
}

fn animate(
    mut query: Query<(&AnimIndices, &mut AnimTimer, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (indicies, mut timer, mut sprite) in query.iter_mut() {
        if indicies.stopped {
            continue;
        }
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if sprite.index < indicies.first {
                sprite.index = indicies.first
            }
            if sprite.index >= indicies.last {
                sprite.index = indicies.first
            } else {
                sprite.index += 1;
            }
        }
    }
}

//fn calc_trans(tile_size)
