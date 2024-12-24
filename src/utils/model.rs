use serde::{Deserialize, Serialize};

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
