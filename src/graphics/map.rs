use std::fs;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    math::Vec3,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, IntoSystemConfigs, OnEnter, OnExit, Query, Res, Resource, Transform, With
    },
    sprite::{Sprite, SpriteBundle}, window::{CursorGrabMode, PrimaryWindow, Window, WindowResized},
};
use serde::Deserialize;
use crate::maze::{player_simulation::{camera_view_toggle, player_movement, PlayerMovement}, textures::rotate_sky};
use crate::maze::maze::maze_setup;
// use crate::maze::player_simulation::camera_controller;
use crate::maze::models::{CameraState, TreeParams};

use super::{show_fps::{setup_fps_ui, update_fps_ui}, states::GameState};

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
        .init_resource::<PlayerMovement>()
        .add_systems(OnEnter(GameState::Game), setup_mouse)
        .add_systems(OnEnter(GameState::Game), maze_setup)
        .add_systems(OnEnter(GameState::Game), display_minimap.after(maze_setup))
        .add_systems(Update, (
            player_movement,
            camera_view_toggle,
            rotate_sky,
        ).chain())
        .add_systems(OnExit(GameState::Game), cleanup_maze)
        .add_systems(Startup, setup_fps_ui)
        .add_systems(Update, (update_fps_ui, update_minimap));
}
}

// Nouveau système pour configurer la souris
fn setup_mouse(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

// Système pour nettoyer la configuration de la souris lors de la sortie du jeu
fn _cleanup_mouse(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

#[derive(Resource, Default)]
pub struct MazeState {
    pub is_ready: bool, 
}
fn display_minimap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
    maze_state: Res<MazeState>,
) {
    if !maze_state.is_ready {
        return;
    }

    let wall_texture = asset_server.load("white_maze.png");
    let floor_texture = asset_server.load("black_maze.png");

    let file_content = fs::read_to_string("src/utils/maps.json").expect("Unable to read file");
    let maze: Maze = serde_json::from_str(&file_content).expect("Unable to parse JSON");

    let height = maze.maze_1.len();
    let width = maze.maze_1[0].len();

    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let minimap_scale = 0.4;

    let minimap_width = TILE_SIZE * width as f32 * minimap_scale;
    let minimap_height = TILE_SIZE * height as f32 * minimap_scale;

    let x_offset = window_width / 2.0 - minimap_width - 20.0; 
    let y_offset = -window_height / 2.0 + minimap_height + 20.0;

    // Générer la minimap
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
    if !maze_state.is_ready {
        return; 
    }

    if resize_events.is_empty() {
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