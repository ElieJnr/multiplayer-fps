use std::{collections::HashMap, net::SocketAddr};

use bevy::{math::Vec3, prelude::Resource};

use crate::{
    player::model::PlayerMovement,
    utils::logger::{display_info, display_warning},
};

#[derive(Resource, Debug)]
pub struct Players(pub HashMap<String, Player>);

// #[derive(Debug, Clone, Resource)]
#[derive(Debug, Clone, Resource, serde::Deserialize, serde::Serialize)]
pub struct Player {
    pub name: String,
    pub address: SocketAddr,
    pub health: u32,
    pub ready: bool,
    pub movement: PlayerMovement,
}

pub fn add_player(
    players: &mut HashMap<String, Player>,
    name: String,
    address: SocketAddr,
    initial_position: Vec3,
) {
    if players.contains_key(&name) {
        display_warning(&format!("Player '{}' is already connected.", name));
    } else {
        players.insert(
            name.clone(),
            Player {
                name: name.clone(),
                address,
                health: 3,
                ready: false,
                movement: PlayerMovement {
                    speed: 5.0,
                    mouse_sensitivity: 0.003,
                    ground_level: 0.0,
                    position: initial_position,
                    rotation: Default::default(),
                    last_processed_input: Default::default(),
                    input_buffer: Default::default(),
                },
            },
        );
        display_info(&format!("{} has joined", name.clone()));
        display_connected_players(players);
    }
}

pub fn remove_player(players: &mut HashMap<String, Player>, name: &str) {
    if players.remove(name).is_some() {
        display_info(&format!("Player '{}' has been removed.", name));
    } else {
        display_warning(&format!("Player '{}' not found.", name));
    }
}

pub fn display_connected_players(players: &HashMap<String, Player>) {
    if players.is_empty() {
        display_info("No players connected.");
    } else {
        display_info("Connected players:");
        for player in players.values() {
            println!(
                "Name: {}, Address: {}, Health: {}",
                player.name, player.address, player.health
            );
        }
    }
}
