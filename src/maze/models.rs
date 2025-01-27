use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
pub struct ProceduralTree;

#[derive(Component)]
pub struct Sky;

#[derive(Component)]
pub struct MinimapCamera;

#[derive(Component)]
pub struct Arch;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component, Resource)]
pub struct PlayersComponent;

#[derive(Default, Resource)]
pub struct CameraState {
    pub is_top_view: bool,
}

#[derive(Debug)]
pub struct Pillar {
    pub position: Vec3,
    pub size: Vec3,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct ColliderEnemy;

#[derive(Component)]
pub struct ColliderHouse;

#[derive(Component)]
pub struct ColliderPillar;

#[derive(Default, Resource, Clone)]
pub struct ObstaclePositions {
    pub positions: Vec<Vec<bool>>,
}

#[derive(Deserialize, Resource)]
pub struct Maze {
    #[serde(rename = "maze-1")]
    pub maze_1: Vec<Vec<i32>>,
}

#[derive(Resource, Default)]
pub struct MazeState {
    pub is_ready: bool,
}

#[derive(Component)]
pub struct MinimapPlayer;

pub struct Branch(pub Transform, pub Option<usize>, pub bool);

#[derive(Debug, Resource)]
pub struct TreeParams {
    pub children: u8,
    pub levels: u8,
    pub child_translation_factor: f32,
    pub angle_from_parent_branch: f32,
    pub child_scale: f32,
    pub base_radius: f32,
    pub leaf_radius: f32,
}

pub struct Textures {
    pub wall_texture: Handle<Image>,
    pub arch_texture: Handle<Image>,
    pub floor_texture: Handle<Image>,
    pub sky_texture: Handle<Image>,
    pub house_textures: HouseTextures,
}

pub struct HouseTextures {
    pub house_1: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_2: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_3: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_4: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_5: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_6: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_7: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    pub house_8: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
}

#[derive(Default)]
pub struct HouseMaterials {
    pub house_1: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_2: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_3: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_4: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_5: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_6: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_7: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
    pub house_8: (
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<StandardMaterial>,
    ),
}

impl Default for TreeParams {
    fn default() -> Self {
        Self {
            children: 3,
            levels: 5,
            child_translation_factor: 1.0,
            angle_from_parent_branch: 0.45,
            child_scale: 0.72,
            base_radius: 0.1,
            leaf_radius: 0.4,
        }
    }
}

#[derive(Resource)]
pub struct MinimapTextures {
    pub wall_texture: Handle<Image>,
    pub floor_texture: Handle<Image>,
    pub player_texture: Handle<Image>,
}
