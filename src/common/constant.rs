use std::collections::HashMap;
use bevy::prelude::Event;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;

use crate::client::player::Player;
use crate::maze::barre_etat::GameStatus;

pub const PORT: &str = "8080";
// pub const MIN_PLAYERS: usize = 1; 

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

#[derive(Event)]
pub struct PlayerCountUpdateEvent {
    pub count: usize,
}

#[derive(Resource)]
pub struct NetworkPlayerState {
    pub connected_players: HashMap<String, Player>,
    pub min_players: usize,
}

impl Default for NetworkPlayerState {
    fn default() -> Self {
        Self {
            connected_players: HashMap::new(),
            min_players: DEFAULT_MIN_PLAYERS,
        }
    }
}

pub fn sync_network_to_game_status(
    network_state: Res<NetworkPlayerState>,
    mut game_status: ResMut<GameStatus>,
) {
    game_status.num_players = network_state.min_players;
    game_status.player_restant = network_state.connected_players.len();
}

pub fn update_network_state(
    mut network_state: ResMut<NetworkPlayerState>,
    players: &HashMap<String, Player>,
) {
    network_state.connected_players = players.clone();
}