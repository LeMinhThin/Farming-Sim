use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ecs_ldtk::prelude::*;
use bevy_xpbd_2d::prelude::*;

mod components;
mod player;

use components::*;
use player::PlayerPlugin;

pub const TILE_SIZE: f32 = 16.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_entities)
        .add_systems(Update, animate)
        .init_resource::<SelectedCell>()
        .insert_resource(LevelSelection::index(0))
        .insert_resource(Gravity::ZERO)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_plugins(PlayerPlugin)
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
