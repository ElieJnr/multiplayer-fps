use std::{collections::HashMap, net::SocketAddr};

use crate::utils::logger::{display_info, display_warning};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub address: SocketAddr,
    pub health: u32,
}

pub fn add_player(players: &mut HashMap<String, Player>, name: String, address: SocketAddr) {
    if players.contains_key(&name) {
        display_warning(&format!("Player '{}' is already connected.", name));
    } else {
        players.insert(
            name.clone(),
            Player {
                name: name.clone(),
                address,
                health: 3,
            },
        );
        display_info(&format!("{} has joined", name.clone()));
    }
}

pub fn remove_player(players: &mut HashMap<String, Player>, name: &str) {
    if players.remove(name).is_some() {
        display_info(&format!("Player '{}' has been removed.", name));
    } else {
        display_warning(&format!("Player '{}' not found.", name));
    }
}
