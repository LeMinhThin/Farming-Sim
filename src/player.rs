use std::usize;

use bevy::transform::TransformSystem;

use crate::*;

const SPEED: f32 = 100.;

fn update_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut player_anim: Query<(&mut AnimIndices, &mut TextureAtlasSprite), With<Player>>,
) {
    let Ok((mut anim, mut atlas)) = player_anim.get_single_mut() else {
        return;
    };
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut accel = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        accel.y += 1.;
        set_row(&mut anim, &mut atlas, 2)
    }
    if input.pressed(KeyCode::S) {
        accel.y -= 1.;
        set_row(&mut anim, &mut atlas, 1)
    }
    if input.pressed(KeyCode::A) {
        accel.x -= 1.;
        set_row(&mut anim, &mut atlas, 3)
    }
    if input.pressed(KeyCode::D) {
        accel.x += 1.;
        set_row(&mut anim, &mut atlas, 4)
    }

    if accel == Vec2::ZERO {
        let sprite_count = anim.last - anim.first + 1;
        let sprite_offset = (anim.row - 1) * sprite_count;
        atlas.index = anim.first + sprite_offset;
        anim.stopped = true;
        player.0 = Vec2::ZERO;
    } else {
        player.0 = accel.normalize() * SPEED;
        anim.stopped = false
    }
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    for mut camera in &mut camera {
        let delta_pos = player.translation - camera.translation;
        camera.translation += delta_pos * 0.1;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player).add_systems(
            PostUpdate,
            move_camera
                .after(PhysicsSet::Sync)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

fn set_row(indicies: &mut AnimIndices, sprite: &mut TextureAtlasSprite, new_row: usize) {
    if new_row == indicies.row {
        return;
    } else {
        let current_index = sprite.index - indicies.offset();
        indicies.row = new_row;
        sprite.index = current_index + indicies.offset();
    }
}
