use bevy::{prelude::*, transform::TransformSystem};
use crate::components::*;
use bevy_xpbd_2d::prelude::*;

const SPEED: f32 = 100.;

fn update_player(mut player: Query<&mut LinearVelocity, With<Player>>, input: Res<Input<KeyCode>>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let mut accel = Vec2::ZERO;

    if input.pressed(KeyCode::W) {
        accel.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        accel.y -= 1.;
    }
    if input.pressed(KeyCode::A) {
        accel.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        accel.x += 1.;
    }

    player.0 = accel.normalize_or_zero() * SPEED;
}

fn update_player_anim(
    player: Query<&LinearVelocity, With<Player>>,
    mut player_sprite: Query<(&mut AnimIndices, &mut TextureAtlasSprite), With<Player>>,
) {
    let Ok(player_vel) = player.get_single() else {
        return;
    };
    let Ok((mut anim, mut atlas)) = player_sprite.get_single_mut() else {
        return;
    };

    let mut flip = atlas.flip_x;
    let mut spr_row = anim.row;

    if player_vel.y > 0. {
        spr_row = 1;
        flip = false;
    }
    if player_vel.y < 0. {
        spr_row = 0;
        flip = false;
    }
    if player_vel.x > 0. {
        spr_row = 2;
        flip = true;
    }
    if player_vel.x < 0. {
        spr_row = 2;
        flip = false;
    }

    anim.row = spr_row;
    atlas.flip_x = flip;
    if player_vel.length() < 1. {
        anim.stopped = true;
        atlas.index = anim.spr_count() * anim.row;
    } else {
        atlas.index = anim.spr_count() * anim.row + atlas.index % anim.spr_count();
        anim.stopped = false;
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

fn update_state(
    mut next_state: ResMut<NextState<PlayerState>>,
    mut player: Query<
        (
            Entity,
            &mut AnimIndices,
            &mut TextureAtlasSprite,
            &mut LinearVelocity,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
) {
    let Ok((player_id, mut anim, mut atlas, mut vel)) = player.get_single_mut() else {
        return;
    };
    if input.just_pressed(MouseButton::Left) {
        let action_timer = ActionTimer(Timer::from_seconds(0.5, TimerMode::Once));

        anim.row += 3;
        atlas.index = anim.spr_count() * anim.row;
        anim.stopped = false;
        vel.x = 0.;
        vel.y = 0.;

        next_state.set(PlayerState::UsingTool);
        commands.entity(player_id).insert(action_timer);
    }
}

fn use_tool(
    mut next_state: ResMut<NextState<PlayerState>>,
    mut player: Query<
        (
            &mut ActionTimer,
            &mut AnimIndices,
            &mut TextureAtlasSprite,
            Entity,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok((mut timer, mut anim, mut atlas, player_id)) = player.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());
    if timer.0.percent() > 0.5 {
        // Really weird bug
        // If the operation below was remove, the player's animation wouldn't play
        // using let _ = as per the complier's help message will break the animation
        //3 + 1;
    }
    if timer.0.just_finished() {
        commands.entity(player_id).remove::<ActionTimer>();
        next_state.set(PlayerState::Normal);
        anim.row -= 3;
        atlas.index = anim.spr_count() * anim.row;
    }
}

fn is_using_tool(player_state: Res<State<PlayerState>>) -> bool {
    return match player_state.get() {
        PlayerState::UsingTool => true,
        _ => false,
    };
}

fn is_normal(player_state: Res<State<PlayerState>>) -> bool {
    return match player_state.get() {
        PlayerState::Normal => true,
        _ => false,
    };
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .add_systems(
                Update,
                (update_player, update_player_anim, update_state).run_if(is_normal),
            )
            .add_systems(Update, use_tool.run_if(is_using_tool))
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
