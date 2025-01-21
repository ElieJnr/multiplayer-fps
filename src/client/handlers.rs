use crate::graphics::resources::PlayerCountState;
use crate::graphics::start::start;
use crate::maze::maze::PosStruct;
use crate::{common::protocol::*, utils::logger::*};

// use super::player::Players;

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
pub fn handle_waiting(
    content: MessageContent,
    config: &NetworkConfig,
    state: &mut PlayerCountState,
) {
    if let MessageContent::WaitForPlayers { msg, players } = content {
        state.has_enough_players = false;

        display_info(&msg);

        let pos=PosStruct{
            position: players.get(&config.player_name.clone()).unwrap().movement.position,
        };

        unsafe {
            start(
                config.player_name.clone(),
                config.server_address.clone(),
                config.client_socket.clone(),
                state.clone(),
                pos,
            );
            GAME_STARTED = true;
        }
    } else {
        display_error("Received invalid content type for WaitForPlayers");
    }
}

pub fn handle_start(content: MessageContent, config: &NetworkConfig, state: &mut PlayerCountState) {
    if let MessageContent::StartGame { msg ,players} = content {
        if msg == "start" {
            state.has_enough_players = true;
        }
        display_info(&msg);

        let pos=PosStruct{
            position: players.get(&config.player_name.clone()).unwrap().movement.position,
        };

        unsafe {
            start(
                config.player_name.clone(),
                config.server_address.clone(),
                config.client_socket.clone(),
                state.clone(),
                pos,
            );
            GAME_STARTED = true;
        }
    } else {
        display_error("Received invalid content type for StartGame");
    }
}
