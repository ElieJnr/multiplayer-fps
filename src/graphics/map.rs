use std::fs;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    math::Vec3,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnEnter, OnExit, Query, Res,
        Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
};
use serde::Deserialize;

use crate::server::maze::{camera_controller, maze_setup, rotate_sky, CameraState, TreeParams};

use super::states::GameState;

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
            .add_systems(OnEnter(GameState::Game), maze_setup)
            .add_systems(Update, (camera_controller, rotate_sky).chain())
            .add_systems(OnExit(GameState::Game), cleanup_maze)
            .add_systems(Startup, display_minimap);
        
    }
}

// pub fn setup_maze() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, maze_setup)
//         .run();
// }

// his function generates a graphical minimap based on JSON data for a maze, with specific textures for the wall, floor and player.
fn display_minimap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file_content = fs::read_to_string("src/utils/maps.json").expect("Unable to read file");
    let maze: Maze = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    // println!("maze {:?}", maze);
    let height = maze.maze_1.len();
    let width = maze.maze_1[0].len();
    let wall_texture = asset_server.load("white_maze.png");
    let floor_texture = asset_server.load("black_maze.png");
    // let player_position = asset_server.load("player.png");
    let x_offset = width as f32 * TILE_SIZE / 2.0 + 100.;
    let y_offset = height as f32 * TILE_SIZE / 2.0 - 200.0;
    for y in 0..height {
        for x in 0..width {
            let texture = if maze.maze_1[y][x] == 2 || maze.maze_1[y][x] == 0  {
                floor_texture.clone()
            } else {
                wall_texture.clone()
            };
            commands.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * TILE_SIZE + x_offset,
                        -(y as f32) * TILE_SIZE + y_offset,
                        0.0,
                    ),
                    scale: Vec3::splat(TILE_SIZE / TEXTURE_SIZE),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn cleanup_maze(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}