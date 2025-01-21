use std::collections::HashMap;

use crate::maze::barre_etat::GameStatusPlugin;
use crate::maze::maze::{maze_setup, setup_crosshair, PosStruct};
use crate::maze::minimap::minimap::{display_minimap, update_minimap};
use crate::maze::minimap::minimap_player::update_minimap_player;
use crate::maze::models::{CameraState, MazeState, ObstaclePositions, TreeParams};
use crate::maze::textures::rotate_sky;
use crate::player::model::RemotePlayers;
use crate::player::model::PlayerMovement;
use crate::player::movement::{camera_view_toggle, player_movement, manage_remote_players, toggle_cursor_lock};
use bevy::{
    app::{App, Plugin, Update},
    prelude::{
        Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnEnter, OnExit, Query, With,
    },
    sprite::Sprite,
    window::{CursorGrabMode, Window},
};

use super::states::GameState;

pub struct MazePlugin;
impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeParams>()
            .init_resource::<CameraState>()
            .init_resource::<MazeState>()
            .init_resource::<PlayerMovement>()
            .init_resource::<PosStruct>()
            .insert_resource(RemotePlayers(HashMap::new()))
            .init_resource::<ObstaclePositions>()
            .add_systems(OnEnter(GameState::Game), setup_mouse)
            .add_systems(OnEnter(GameState::Game), (maze_setup, setup_crosshair))
            .add_systems(OnEnter(GameState::Game), display_minimap.after(maze_setup))
            .add_systems(
                Update,
                (
                    player_movement,
                    manage_remote_players,
                    camera_view_toggle,
                    rotate_sky,
                    update_minimap_player,
                ),
            )
            .add_systems(OnExit(GameState::Game), cleanup_maze)
            .add_systems(Update, (update_minimap, toggle_cursor_lock));

        app.add_plugins(GameStatusPlugin);
    }
}

// Nouveau système pour configurer la souris
fn setup_mouse(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

fn cleanup_maze(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
