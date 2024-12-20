use std::net::{SocketAddr, UdpSocket};

use serde::{Deserialize, Serialize};

use crate::utils::logger::*;


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
        },
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

pub fn receive_data_from_socket(socket: &UdpSocket, buf: &mut [u8]) -> Option<(Vec<u8>, Option<SocketAddr>)> {
    match socket.recv_from(buf) {
        Ok((size, src)) => {
            let data = buf[..size].to_vec();
            Some((data, Some(src))) 
        },
        Err(err) => {
            display_error(&format!("Failed to receive data: {}", err));
            None
        }
    }
}