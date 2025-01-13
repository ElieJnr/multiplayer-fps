use bevy::{
    asset::Handle,
    color::Color,
    math::{EulerRot, Quat, Vec3},
    prelude::{Commands, Image, Query, Res, Transform, With, Without},
    sprite::{Sprite, SpriteBundle},
    window::{PrimaryWindow, Window},
};

use crate::{
    maze::{
        maze::calculate_maze_dimensions,
        models::{Maze, MinimapPlayer, Player},
    },
    utils::utils::get_window_dimensions,
};

use super::minimap::{
    calculate_minimap_dimensions, calculate_minimap_offsets, MINIMAP_SCALE, TILE_SIZE,
};

pub fn spawn_minimap_player(
    mut commands: Commands<'_, '_>,
    player_texture: Handle<Image>,
    minimap_scale: f32,
    x_offset: f32,
    y_offset: f32,
) {
    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            transform: Transform {
                translation: Vec3::new(x_offset, y_offset, 1.0),
                scale: Vec3::splat(TILE_SIZE * minimap_scale * 0.01),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::srgb(255.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapPlayer,
    ));
}

pub fn update_minimap_player(
    mut minimap_query: Query<&mut Transform, With<MinimapPlayer>>,
    player_query: Query<&Transform, (With<Player>, Without<MinimapPlayer>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    map: Res<Maze>,
) {
    let (window_width, window_height) = get_window_dimensions(&windows);

    if let (Ok(mut minimap_transform), Ok(player_transform)) =
        (minimap_query.get_single_mut(), player_query.get_single())
    {
        let maze = &map.maze_1;

        let (height, width) = calculate_maze_dimensions(&maze);
        let (minimap_width, minimap_height) =
            calculate_minimap_dimensions(height, width, MINIMAP_SCALE);
        let (base_x_offset, base_y_offset) =
            calculate_minimap_offsets(window_width, window_height, minimap_width, minimap_height);

        let (player_minimap_x, player_minimap_y) = calculate_player_minimap_position(
            &player_transform,
            MINIMAP_SCALE,
            base_x_offset,
            base_y_offset,
        );

        update_player_minimap_rotation(
            &mut minimap_transform,
            player_minimap_x,
            player_minimap_y,
            &player_transform,
        );
    }
}

fn calculate_player_minimap_position(
    player_transform: &Transform,
    minimap_scale: f32,
    base_x_offset: f32,
    base_y_offset: f32,
) -> (f32, f32) {
    let grid_x = player_transform.translation.x;
    let grid_z = player_transform.translation.z;

    let player_minimap_x = grid_x * (TILE_SIZE * minimap_scale) + base_x_offset;
    let player_minimap_y = -grid_z * (TILE_SIZE * minimap_scale) + base_y_offset;

    (player_minimap_x, player_minimap_y)
}

fn update_player_minimap_rotation(
    minimap_transform: &mut Transform,
    player_minimap_x: f32,
    player_minimap_y: f32,
    player_transform: &Transform,
) {
    minimap_transform.translation.x = player_minimap_x;
    minimap_transform.translation.y = player_minimap_y;
    minimap_transform.translation.z = 1.0;

    let player_angle = -player_transform.rotation.to_euler(EulerRot::XYZ).1;
    minimap_transform.rotation = Quat::from_rotation_z(player_angle);
}
