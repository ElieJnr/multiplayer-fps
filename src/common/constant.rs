use bevy::prelude::Resource;

pub const PORT: &str = "8080";
pub const HEALTH_NBR: u32 = 5;

pub const PLAYER_URL: &str =
    "https://raw.githubusercontent.com/Baabacar/player_model/main/player.glb";
pub const ENEMY_URL: &str =
    "https://raw.githubusercontent.com/Baabacar/player_model/main/enemy.glb";
pub const PLAYER_PATH: &str = "assets/player.glb";
pub const ENEMY_PATH: &str = "assets/enemy.glb";

pub const DEFAULT_MIN_PLAYERS: usize = 2;
#[derive(Debug, Resource, Default)]
pub struct PlayerCount {
    min_players: usize,
}

impl PlayerCount {
    pub fn new(min_players: usize) -> Self {
        PlayerCount { min_players }
    }

    pub fn get_min_players(&self) -> usize {
        self.min_players
    }

    pub fn set_min_players(&mut self, min_players: usize) {
        self.min_players = min_players;
    }
}
