use crate::{structs::*, TILE_SIZE};

use bevy::math::vec2;
use bevy::{prelude::*, transform::TransformSystem, window::PrimaryWindow};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_xpbd_2d::prelude::*;

const SPEED: f32 = 100.;

fn move_player(mut player: Query<&mut LinearVelocity, With<Player>>, input: Res<Input<KeyCode>>) {
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
    input: Res<Input<KeyCode>>,
    mut player_sprite: Query<(&mut AnimIndices, &mut TextureAtlasSprite), With<Player>>,
) {
    let Ok((mut anim, mut atlas)) = player_sprite.get_single_mut() else {
        return;
    };

    let mut flip = atlas.flip_x;
    let mut row = 10;

    if input.pressed(KeyCode::W) {
        row = 1;
        flip = false;
    }
    if input.pressed(KeyCode::S) {
        row = 0;
        flip = false;
    }
    if input.pressed(KeyCode::D) {
        row = 2;
        flip = true;
    }
    if input.pressed(KeyCode::A) {
        row = 2;
        flip = false;
    }

    if row == 10 {
        anim.stopped = true;
        atlas.index = anim.offset();
        return;
    }
    anim.set_row(row);
    anim.stopped = false;
    atlas.flip_x = flip;
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
        let action_timer = ActionTimer(Timer::from_seconds(0.5, TimerMode::Once), false);
        let current_row = anim.row();

        anim.stopped = false;
        anim.set_row(current_row + 3);
        atlas.index = anim.spr_count() * current_row;
        vel.x = 0.;
        vel.y = 0.;

        next_state.set(PlayerState::UsingTool);
        commands.entity(player_id).insert(action_timer);
    }
}

fn use_tool(
    mut player: Query<&mut ActionTimer, With<Player>>,
    mut till_event: EventWriter<TillSoilEvent>,
    time: Res<Time>,
    selected_cell: Res<SelectedCell>,
) {
    let Ok(mut action) = player.get_single_mut() else {
        return;
    };
    let Some(cell) = selected_cell.0 else {
        return;
    };
    let cell = cell / TILE_SIZE;
    action.0.tick(time.delta());
    if action.0.percent() > 0.5 && !action.1 {
        till_event.send(TillSoilEvent(TilePos::new(cell.x as u32, cell.y as u32)));
        action.1 = true
    }
}

fn tool_anim(
    mut commands: Commands,
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
    time: Res<Time>,
) {
    let Ok((mut timer, mut anim, mut atlas, player_id)) = player.get_single_mut() else {
        return;
    };
    timer.0.tick(time.delta());

    if timer.0.finished() {
        commands.entity(player_id).remove::<ActionTimer>();
        next_state.set(PlayerState::Normal);
        let row = anim.row();
        anim.set_row(row - 3);
        atlas.index = anim.spr_count() * anim.row();
    }
}

fn is_using_tool(player_state: Res<State<PlayerState>>) -> bool {
    matches!(player_state.get(), PlayerState::UsingTool)
}

fn is_normal(player_state: Res<State<PlayerState>>) -> bool {
    matches!(player_state.get(), PlayerState::Normal)
}

fn update_selected_cell(
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
    player_pos: Query<&Transform, With<Player>>,
    mut selected_cell: ResMut<SelectedCell>,
) {
    let Ok(player) = player_pos.get_single() else {
        return;
    };
    let window = window.single();
    let (camera, glob_trans) = camera.single();
    let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(glob_trans, cursor))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };

    let map_coord = ((world_pos / TILE_SIZE).trunc() * TILE_SIZE) + vec2(TILE_SIZE, TILE_SIZE) / 2.;

    selected_cell.0 = if (map_coord.x - player.translation.x).abs() > TILE_SIZE * 1.5 {
        None
    } else if (map_coord.y - player.translation.y).abs() > TILE_SIZE * 1.5 {
        None
    } else {
        Some(map_coord)
    }
}

fn draw_selected_cell(coord: Res<SelectedCell>, mut gizmos: Gizmos) {
    let Some(coord) = coord.0 else {
        return;
    };
    gizmos.rect_2d(coord, 0., vec2(TILE_SIZE, TILE_SIZE), Color::BLACK);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .add_systems(
                Update,
                (move_player, update_player_anim, update_state)
                    .chain()
                    .run_if(is_normal),
            )
            .add_systems(Update, (use_tool, tool_anim).run_if(is_using_tool))
            .add_systems(Update, (update_selected_cell, draw_selected_cell).chain())
            .add_systems(
                PostUpdate,
                move_camera
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}
