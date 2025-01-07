use bevy::app::Update;
use bevy::math::Vec3;
use bevy::prelude::App;
use bevy::prelude::AssetServer;
use bevy::prelude::Camera;
use bevy::prelude::Camera2dBundle;
use bevy::prelude::Commands;
use bevy::prelude::Plugin;
use bevy::prelude::Res;
use bevy::prelude::Startup;
use bevy::prelude::Transform;
use bevy::sprite::SpriteBundle;
use serde::Deserialize;
use std::fs;

use crate::server::maze::MinimapCamera;

const TILE_SIZE: f32 = 6.0;
const TEXTURE_SIZE: f32 = 24.0;

#[derive(Deserialize, Debug)]
struct Maze {
    #[serde(rename = "maze-1")]
    maze_1: Vec<Vec<u8>>,
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_maze, display_minimap));
    }
}

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

    let x_offset = width as f32 * TILE_SIZE / 2.0 + 200.;
    let y_offset = height as f32 * TILE_SIZE / 2.0 - 300.0;

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

fn setup_maze(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0), // Ensure it can view the minimap
            camera: Camera {
                order: 1, // Ensure this camera renders on top of the 3D scene
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapCamera, // Marker component for identification
    ));
}


