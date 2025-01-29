use super::movement::*;
use super::player::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct PlayerBuild;

#[derive(Component)]
pub struct AnimationState {
    pub current_animation: String,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: "idle".to_string(),
        }
    }
}
#[derive(Resource)]
pub struct PreloadedPlayerAnimations {
    pub model: Handle<Scene>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct PreloadedEnemyAnimations {
    pub model: Handle<Scene>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub player_entity: Entity,
    pub animation_player_entity: Option<Entity>,
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Resource)]
pub struct EnemyAnimations {
    pub player_entity: Entity,
    pub animation_player_entity: Option<Entity>,
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct PlayerComponent;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, preload_player_assets)
            .add_systems(Update, (setup_player_animation, handle_keyboard_animation, update_bullets));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default, Component, Resource)]
pub struct RemotePlayer {
    pub name: String,
}

#[derive(Resource)]
pub struct RemotePlayers(pub HashMap<String, Entity>);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerInput {
    pub arrow_up: bool,
    pub arrow_down: bool,
    pub arrow_left: bool,
    pub arrow_right: bool,
    pub mouse_delta: Vec2,
    pub ready: bool,
    pub shoot: bool
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InputSequence {
    pub sequence_number: u32,
    pub timestamp: f64,
    pub input: PlayerInput,
}

#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct PlayerMovement {
    pub speed: f32,
    pub mouse_sensitivity: f32,
    pub ground_level: f32,
    pub position: Vec3,
    pub rotation: Vec2,
    pub last_processed_input: u32,
    pub input_buffer: VecDeque<InputSequence>,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            mouse_sensitivity: 0.003,
            ground_level: 1.0,
            position: Vec3::ZERO,
            rotation: Vec2::ZERO,
            last_processed_input: 0,
            input_buffer: VecDeque::new(),
        }
    }
}

impl PlayerMovement {
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
    }
}

#[derive(Component, Resource)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

#[derive(Component, Resource)]
pub struct SmokeParticle {
    pub lifetime: f32,
}
