use crate::maze::maze::maze_setup;
use crate::maze::minimap::minimap::{display_minimap, update_minimap, update_minimap_player};
use crate::maze::models::{CameraState, MazeState, TreeParams};
use crate::maze::{
    player_simulation::{camera_view_toggle, player_movement, PlayerMovement},
    textures::rotate_sky,
};
use bevy::{
    app::{App, Plugin, Startup, Update},
    prelude::{
        Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnEnter, OnExit, Query, With,
    },
    sprite::Sprite,
    window::{CursorGrabMode, Window},
};

use super::{
    show_fps::{setup_fps_ui, update_fps_ui},
    states::GameState,
};

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
            .add_systems(
                Update,
                (
                    player_movement,
                    camera_view_toggle,
                    rotate_sky,
                    update_minimap_player,
                )
                    .chain(),
            )
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

fn cleanup_maze(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
