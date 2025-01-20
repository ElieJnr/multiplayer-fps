use crate::client::player::Player;
use crate::{maze::player_simulation::PlayerInput, utils::logger::*};
use bevy::{math::Vec2, prelude::Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

#[derive(Resource, Debug)]
pub struct NetworkConfig {
    pub player_name: String,
    pub server_address: String,
    pub client_socket: Arc<UdpSocket>,
}

impl NetworkConfig {
    pub fn new(player_name: String, server_address: String, socket: UdpSocket) -> Self {
        NetworkConfig {
            player_name,
            server_address,
            client_socket: Arc::new(socket),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameMessage {
    pub message_type: MessageType,
    pub sender: String,
    pub content: MessageContent,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum MessageType {
    NewConnection,
    GameUpdate,
    PlayerAction,
    ServerInfo,
    Disconnect,
    WaitForPlayers,
    StartGame,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageContent {
    NewConnection {
        name: String,
    },
    GameUpdate {
        position: (f32, f32),
        rotation: Vec2,
        sequence_number: u32,
        timestamp: f64,
    },
    PlayerAction {
        action: PlayerInput,
        sequence_number: u32,
        timestamp: f64,
    },
    ServerInfo {
        server_status: String,
    },
    Disconnect {
        reason: String,
    },
    WaitForPlayers {
        msg: String,
        players: HashMap<String, Player>,
    },
    StartGame {
        msg: String,
        players: HashMap<String, Player>,
    },
}

pub fn serialize_message(message: &GameMessage) -> Option<Vec<u8>> {
    match serde_json::to_string(message) {
        Ok(json) => {
            let msg_bytes = json.as_bytes().to_vec();
            Some(msg_bytes)
        }
        Err(err) => {
            display_error(&format!("Failed to serialize message: {}", err));
            None
        }
    }
}

pub fn deserialize_message(data: &[u8]) -> Option<GameMessage> {
    match serde_json::from_slice(data) {
        Ok(message) => Some(message),
        Err(e) => {
            display_error(&format!("Erreur lors de la désérialisation: {}", e));
            None
        }
    }
}

pub fn receive_data_from_socket(
    socket: &UdpSocket,
    buf: &mut [u8],
) -> Option<(Vec<u8>, Option<SocketAddr>)> {
    match socket.recv_from(buf) {
        Ok((size, src)) => {
            let data = buf[..size].to_vec();
            Some((data, Some(src)))
        }
        Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => None,
        Err(err) => {
            display_error(&format!("Failed to receive data: {}", err));
            None
        }
    }
}

pub fn send_disconnect_message(config: &NetworkConfig, player_name: &str, reason: &str) {
    let disconnect_message = GameMessage {
        message_type: MessageType::Disconnect,
        sender: player_name.to_string(),
        content: MessageContent::Disconnect {
            reason: reason.to_string(),
        },
    };

    let serialized_msg = match serialize_message(&disconnect_message) {
        Some(msg) => msg,
        None => return display_error("Failed to serialize disconnect message."),
    };

    if let Err(err) = config
        .client_socket
        .send_to(&serialized_msg, config.server_address.clone())
    {
        display_error(&format!("Failed to send disconnect message: {}", err));
    } else {
        display_info(&format!("{} is disconnected successfully.", player_name));
    }
}

pub fn send_ready_msg(config: &NetworkConfig, player_name: &str) {
    let ready_msg = GameMessage {
        message_type: MessageType::PlayerAction,
        sender: player_name.to_string(),
        content: MessageContent::PlayerAction {
            action: PlayerInput {
                arrow_up: false,
                arrow_down: false,
                mouse_delta: Vec2::default(),
                ready: true,
            },
            sequence_number: Default::default(),
            timestamp: Default::default(),
        },
    };

    let serialized_msg = match serialize_message(&ready_msg) {
        Some(msg) => msg,
        None => return display_error("Failed to serialize disconnect message."),
    };

    if let Err(err) = config
        .client_socket
        .send_to(&serialized_msg, config.server_address.clone())
    {
        display_error(&format!("Failed to send disconnect message: {}", err));
    } else {
        display_info(&format!("{} is disconnected successfully.", player_name));
    }
}

pub fn send_new_connection(config: &NetworkConfig) -> Option<()> {
    let message = GameMessage {
        message_type: MessageType::NewConnection,
        sender: config.player_name.to_string(),
        content: MessageContent::NewConnection {
            name: config.player_name.to_string(),
        },
    };

    let msg_bytes = match serialize_message(&message) {
        Some(bytes) => bytes,
        None => return None,
    };

    if let Err(err) = config.client_socket.send(&msg_bytes) {
        display_error(&format!("Failed to send message: {}", err));
        None
    } else {
        Some(())
    }
}
