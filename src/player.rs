use bevy::transform::TransformSystem;

use crate::*;

const SPEED: f32 = 100.;
//const MAX_SPEED: f32 = 200.;
//const FRICTION: f32 = 7.;

fn move_player(mut player: Query<&mut LinearVelocity, With<Player>>, input: Res<Input<KeyCode>>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut accel = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        accel.y += 1.;
    }
    if input.pressed(KeyCode::A) {
        accel.x -= 1.;
    }
    if input.pressed(KeyCode::S) {
        accel.y -= 1.;
    }
    if input.pressed(KeyCode::D) {
        accel.x += 1.;
    }

    player.0 = accel.normalize_or_zero() * SPEED;
    /*
    if player.0.length() > MAX_SPEED {
        player.0 = player.0.normalize()
    }
    */
}

fn move_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    //let mut camera = camera.single_mut();
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
        app.add_systems(Update, move_player)
            //.add_systems(Update, animate_player)
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
