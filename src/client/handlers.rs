use crate::common::constant::PLAYER_COUNT_STATE;
use crate::graphics::start::start;
use crate::{common::protocol::*, utils::logger::*};
pub fn handle_disconnect(content: MessageContent) {
    if let MessageContent::Disconnect { reason } = content {
        display_warning(&format!("[{}]", reason));
    } else {
        display_warning(&format!("Received invalid content type for NewConnection"));
    }
}

pub fn handle_server_info(content: MessageContent) {
    if let MessageContent::ServerInfo { server_status } = content {
        display_info(&format!("[{}]", server_status));
    } else {
        display_error("Received invalid content type for ServerInfo");
    }
}

pub fn handle_new_connection(content: MessageContent) {
    if let MessageContent::NewConnection { name } = content {
        display_info(&format!("[{} has joined the game.]", name));
    } else {
        display_error("Received invalid content type for NewConnection");
    }
}

static mut GAME_STARTED: bool = false;
pub fn handle_waiting(content: MessageContent, config: &NetworkConfig) {
    if let MessageContent::WaitForPlayers { msg } = content {
        if let Ok(mut state) = PLAYER_COUNT_STATE.lock() {
            state.has_enough_players = false;
        }
        display_info(&msg);
        unsafe {
            if !GAME_STARTED {
                start(config);
                GAME_STARTED = true;
            }
        }
    } else {
        display_error("Received invalid content type for WaitForPlayers");
    }
}

pub fn handle_start(content: MessageContent, config: &NetworkConfig) {
    if let MessageContent::StartGame { msg } = content {
        if let Ok(mut state) = PLAYER_COUNT_STATE.lock() {
            state.has_enough_players = true;
        }
        display_info(&msg);
        unsafe {
            if !GAME_STARTED {
                start(config);
                GAME_STARTED = true;
            }
        }
    } else {
        display_error("Received invalid content type for StartGame");
    }
}
