use std::fs;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Handle},
    math::Vec3,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Image, IntoSystemConfigs, OnEnter,
        OnExit, Query, Res, Resource, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    window::{PrimaryWindow, Window, WindowResized},
};
use serde::Deserialize;

use crate::server::maze::{camera_controller, maze_setup, rotate_sky, CameraState, TreeParams};

use super::{
    show_fps::{setup_fps_ui, update_fps_ui},
    states::GameState,
};

const TILE_SIZE: f32 = 15.0;
const TEXTURE_SIZE: f32 = 114.0;

#[derive(Deserialize)]
struct Maze {
    #[serde(rename = "maze-1")]
    maze_1: Vec<Vec<u8>>,
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeParams>()
            .init_resource::<CameraState>()
            .init_resource::<MazeState>()
            .add_systems(OnEnter(GameState::Game), maze_setup)
            .add_systems(OnEnter(GameState::Game), display_minimap.after(maze_setup))
            .add_systems(Update, (camera_controller, rotate_sky).chain())
            .add_systems(OnExit(GameState::Game), cleanup_maze)
            .add_systems(Startup, setup_fps_ui)
            .add_systems(Update, (update_fps_ui, update_minimap));
    }
}

#[derive(Resource, Default)]
pub struct MazeState {
    pub is_ready: bool,
}

fn display_minimap(
    commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    maze_state: Res<MazeState>,
) {
    if !maze_state.is_ready {
        return;
    }

    let (wall_texture, floor_texture) = load_texture(asset_server);
    let maze = read_map();
    let (height, width) = calculate_maze_dimensions(&maze);
    let (window_width, window_height) = calculate_window_dimensions(&windows);
    let minimap_scale = 0.4;
    let (minimap_width, minimap_height) =
        calculate_minimap_dimensions(height, width, minimap_scale);
    let (x_offset, y_offset) =
        calculate_minimap_offsets(window_width, window_height, minimap_width, minimap_height);

    generate_minimap(
        commands,
        wall_texture,
        floor_texture,
        maze,
        height,
        width,
        minimap_scale,
        x_offset,
        y_offset,
    );
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
    let window_width = window.width();
    let window_height = window.height();
    (window_width, window_height)
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

fn load_texture(asset_server: Res<'_, AssetServer>) -> (Handle<Image>, Handle<Image>) {
    let wall_texture = asset_server.load("white_maze.png");
    let floor_texture = asset_server.load("black_maze.png");
    (wall_texture, floor_texture)
}

fn generate_minimap(
    mut commands: Commands<'_, '_>,
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

fn cleanup_maze(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
