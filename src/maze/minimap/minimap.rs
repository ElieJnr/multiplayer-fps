use std::fs;

use bevy::{
    asset::{AssetServer, Handle},
    color::Color,
    math::{EulerRot, Quat, Vec3},
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Image, Query, Res, Transform, With,
        Without,
    },
    sprite::{Sprite, SpriteBundle},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::maze::models::{Maze, MazeState, MinimapPlayer, Player};

const TILE_SIZE: f32 = 15.0;
const TEXTURE_SIZE: f32 = 114.0;
const MINIMAP_SCALE: f32 = 0.4;

pub fn display_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    maze_state: Res<MazeState>,
) {
    if !maze_state.is_ready {
        return;
    }

    let (wall_texture, floor_texture) = load_texture(&asset_server);
    let player_texture = asset_server.load("player_marker.png");

    let maze = read_map();
    let (height, width) = calculate_maze_dimensions(&maze);
    let (window_width, window_height) = calculate_window_dimensions(&windows);
    let minimap_scale = MINIMAP_SCALE;
    let (minimap_width, minimap_height) =
        calculate_minimap_dimensions(height, width, minimap_scale);
    let (x_offset, y_offset) =
        calculate_minimap_offsets(window_width, window_height, minimap_width, minimap_height);

    generate_minimap(
        &mut commands,
        wall_texture,
        floor_texture,
        maze,
        height,
        width,
        minimap_scale,
        x_offset,
        y_offset,
    );

    spawn_minimap_player(commands, player_texture, minimap_scale, x_offset, y_offset);
}

fn spawn_minimap_player(
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

fn calculate_minimap_offsets(
    window_width: f32,
    window_height: f32,
    minimap_width: f32,
    minimap_height: f32,
) -> (f32, f32) {
    let x_offset = window_width / 2.0 - minimap_width - 20.0;
    let y_offset = -window_height / 2.0 + minimap_height + 20.0;
    (x_offset, y_offset)
}

fn calculate_minimap_dimensions(height: usize, width: usize, minimap_scale: f32) -> (f32, f32) {
    let minimap_width = TILE_SIZE * width as f32 * minimap_scale;
    let minimap_height = TILE_SIZE * height as f32 * minimap_scale;
    (minimap_width, minimap_height)
}

fn calculate_window_dimensions(
    windows: &Query<'_, '_, &Window, With<PrimaryWindow>>,
) -> (f32, f32) {
    let window = windows.single();
    (window.width(), window.height())
}

fn calculate_maze_dimensions(maze: &Maze) -> (usize, usize) {
    let height = maze.maze_1.len();
    let width = maze.maze_1[0].len();
    (height, width)
}

fn read_map() -> Maze {
    let file_content = fs::read_to_string("src/utils/maps.json").expect("Unable to read file");
    let maze: Maze = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    maze
}

fn load_texture(asset_server: &Res<AssetServer>) -> (Handle<Image>, Handle<Image>) {
    let wall_texture = asset_server.load("white_maze.png");
    let floor_texture = asset_server.load("black_maze.png");
    (wall_texture, floor_texture)
}

fn generate_minimap(
    commands: &mut Commands,
    wall_texture: Handle<Image>,
    floor_texture: Handle<Image>,
    maze: Maze,
    height: usize,
    width: usize,
    minimap_scale: f32,
    x_offset: f32,
    y_offset: f32,
) {
    for y in 0..height {
        for x in 0..width {
            let texture = if maze.maze_1[y][x] == 2 || maze.maze_1[y][x] == 0 {
                floor_texture.clone()
            } else {
                wall_texture.clone()
            };

            commands.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * TILE_SIZE * minimap_scale + x_offset,
                        -(y as f32) * TILE_SIZE * minimap_scale + y_offset,
                        0.0,
                    ),
                    scale: Vec3::splat((TILE_SIZE / TEXTURE_SIZE) * minimap_scale),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

pub fn update_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    resize_events: EventReader<WindowResized>,
    sprites: Query<Entity, With<Sprite>>,
    maze_state: Res<MazeState>,
) {
    if !maze_state.is_ready || resize_events.is_empty() {
        return;
    }

    for entity in &sprites {
        commands.entity(entity).despawn_recursive();
    }

    display_minimap(commands, asset_server, windows, maze_state);
}

pub fn update_minimap_player(
    mut minimap_query: Query<&mut Transform, With<MinimapPlayer>>,
    player_query: Query<&Transform, (With<Player>, Without<MinimapPlayer>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let (window_width, window_height) = get_window_dimensions(&windows);

    if let (Ok(mut minimap_transform), Ok(player_transform)) = (minimap_query.get_single_mut(), player_query.get_single()) {
        let minimap_scale = 0.4;
        let maze = read_map();
        let (height, width) = calculate_maze_dimensions(&maze);
        let (minimap_width, minimap_height) = calculate_minimap_dimensions(height, width, minimap_scale);
        let (base_x_offset, base_y_offset) = calculate_minimap_offsets(window_width, window_height, minimap_width, minimap_height);

        let (player_minimap_x, player_minimap_y) = calculate_player_minimap_position(
            &player_transform,
            minimap_scale,
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

fn get_window_dimensions(windows: &Query<&Window, With<PrimaryWindow>>) -> (f32, f32) {
    let window = windows.single();
    (window.width(), window.height())
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
