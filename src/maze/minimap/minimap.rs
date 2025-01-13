use std::fs;

use bevy::{
    asset::{AssetServer, Handle},
    math::Vec3,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Image, Query, Res, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::{
    maze::{
        maze::calculate_maze_dimensions,
        models::{Maze, MazeState, MinimapTextures},
    },
    utils::utils::get_window_dimensions,
};

use super::minimap_player::spawn_minimap_player;

pub const TILE_SIZE: f32 = 15.0;
pub const TEXTURE_SIZE: f32 = 115.0;
pub const MINIMAP_SCALE: f32 = 0.4;
pub const MARGIN: f32 = 20.0;

pub fn display_minimap(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    maze_state: Res<MazeState>,
    textures: Res<MinimapTextures>,
    map: Res<Maze>,
) {
    if !maze_state.is_ready {
        return;
    }

    let maze = &map.maze_1;
    let (height, width) = calculate_maze_dimensions(&maze);
    let (window_width, window_height) = get_window_dimensions(&windows);
    let minimap_scale = MINIMAP_SCALE;
    let (minimap_width, minimap_height) =
        calculate_minimap_dimensions(height, width, minimap_scale);
    let (x_offset, y_offset) =
        calculate_minimap_offsets(window_width, window_height, minimap_width, minimap_height);

    generate_minimap(
        &mut commands,
        textures.wall_texture.clone(),
        textures.floor_texture.clone(),
        maze,
        height,
        width,
        minimap_scale,
        x_offset,
        y_offset,
    );

    spawn_minimap_player(
        commands,
        textures.player_texture.clone(),
        minimap_scale,
        x_offset,
        y_offset,
    );
}

pub fn calculate_minimap_offsets(
    window_width: f32,
    window_height: f32,
    minimap_width: f32,
    minimap_height: f32,
) -> (f32, f32) {
    let x_offset = window_width / 2.0 - minimap_width - MARGIN;
    let y_offset = -window_height / 2.0 + minimap_height + MARGIN;
    (x_offset, y_offset)
}

pub fn calculate_minimap_dimensions(height: f32, width: f32, minimap_scale: f32) -> (f32, f32) {
    let minimap_width = TILE_SIZE * width * minimap_scale;
    let minimap_height = TILE_SIZE * height * minimap_scale;
    (minimap_width, minimap_height)
}

pub fn read_maze(mut commands: Commands) {
    let file_content = fs::read_to_string("src/maze/maze.json").expect("Unable to read file");
    let maze: Maze = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    commands.insert_resource(maze);
}

pub fn load_minimap_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = MinimapTextures {
        wall_texture: asset_server.load("white_maze.png"),
        floor_texture: asset_server.load("black_maze.png"),
        player_texture: asset_server.load("player_marker.png"),
    };

    commands.insert_resource(textures);
}

fn generate_minimap(
    commands: &mut Commands,
    wall_texture: Handle<Image>,
    floor_texture: Handle<Image>,
    maze: &Vec<Vec<i32>>,
    height: f32,
    width: f32,
    minimap_scale: f32,
    x_offset: f32,
    y_offset: f32,
) {
    for y in 0..height as usize{
        for x in 0..width as usize {
            let texture = if maze[y][x] == 2 || maze[y][x] == 0 {
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
    windows: Query<&Window, With<PrimaryWindow>>,
    resize_events: EventReader<WindowResized>,
    sprites: Query<Entity, With<Sprite>>,
    maze_state: Res<MazeState>,
    textures: Res<MinimapTextures>,
    map: Res<Maze>,
) {
    if !maze_state.is_ready || resize_events.is_empty() {
        return;
    }

    for entity in &sprites {
        commands.entity(entity).despawn_recursive();
    }

    display_minimap(commands, windows, maze_state, textures, map);
}
