use std::net::{SocketAddr, UdpSocket};

use serde::{Deserialize, Serialize};

use crate::utils::logger::*;

use super::constant::get_server_address;

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
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum MessageContent {
    NewConnection { name: String },
    GameUpdate { position: (f32, f32), score: u32 },
    PlayerAction { action: String },
    ServerInfo { server_status: String },
    Disconnect { reason: String },
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

pub fn send_disconnect_message(socket: &UdpSocket, player_name: &str, reason: &str) {
    let server_addr = match get_server_address() {
        Some(addr) => addr,
        None => return display_error("Failed to retrieve server address."),
    };

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

    if let Err(err) = socket.send_to(&serialized_msg, server_addr) {
        display_error(&format!("Failed to send disconnect message: {}", err));
    } else {
        display_info(&format!("{} is disconnected successfully.", player_name));
    }
}

pub fn send_new_connection(socket: &UdpSocket, name: &str) -> Option<()> {
    let message = GameMessage {
        message_type: MessageType::NewConnection,
        sender: name.to_string(),
        content: MessageContent::NewConnection {
            name: name.to_string(),
        },
    };

    let msg_bytes = match serialize_message(&message) {
        Some(bytes) => bytes,
        None => return None,
    };

    if let Err(err) = socket.send(&msg_bytes) {
        display_error(&format!("Failed to send message: {}", err));
        None
    } else {
        Some(())
    }
}