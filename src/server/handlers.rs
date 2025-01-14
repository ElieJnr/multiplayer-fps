use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    client::player::{add_player, Player},
    common::{constant::MIN_PLAYERS, protocol::*},
    graphics::resources::PlayerCountState,
    server::udp::broadcast_message,
    utils::logger::*,
};

pub fn handle_message(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    message: GameMessage,
    src: SocketAddr,
    state: &mut PlayerCountState,
) {
    match message.message_type {
        MessageType::NewConnection => {
            handle_new_connection(server_socket, players, message, src, state)
        }
        MessageType::PlayerAction => handle_player_action(message, src),
        MessageType::GameUpdate => handle_game_update(server_socket, players, message),
        MessageType::Disconnect => handle_disconnect(server_socket, players, message, src),
        _ => display_error(&format!(
            "Unhandled message type: {:?}",
            message.message_type
        )),
    }
}

fn handle_new_connection(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    message: GameMessage,
    src: SocketAddr,
    state: &mut PlayerCountState,
) {
    add_player(players, message.sender.clone(), src);
    state.player_count = players.len();
    
    let has_enough = players.len() >= MIN_PLAYERS;
    state.has_enough_players = has_enough;

    send_new_connection_message(server_socket, players, &message.sender);
    if !has_enough {
        send_wait_for_players_message(server_socket, players);
    } else {
        start_game(server_socket, players);
    }
}

fn send_new_connection_message(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    player_name: &str,
) {
    let msg = GameMessage {
        message_type: MessageType::NewConnection,
        sender: "server".to_string(),
        content: MessageContent::NewConnection {
            name: player_name.to_string(),
        },
    };

    if let Some(msg_json) = serialize_message(&msg) {
        broadcast_message(server_socket, players, msg_json, None, true);
    }
}

fn send_wait_for_players_message(server_socket: &UdpSocket, players: &mut HashMap<String, Player>) {
    let msg = GameMessage {
        message_type: MessageType::WaitForPlayers,
        sender: "server".to_string(),
        content: MessageContent::WaitForPlayers {
            msg: "Please wait for other players...".to_string(),
        },
    };

    if let Some(msg_json) = serialize_message(&msg) {
        broadcast_message(server_socket, players, msg_json, None, true);
    }
}

fn start_game(server_socket: &UdpSocket, players: &mut HashMap<String, Player>) {
    let msg = GameMessage {
        message_type: MessageType::StartGame,
        sender: "server".to_string(),
        content: MessageContent::StartGame {
            msg: "Ready for the game".to_string(),
        },
    };

    if let Some(msg_json) = serialize_message(&msg) {
        broadcast_message(server_socket, players, msg_json, None, true);
    }
}

fn handle_disconnect(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    message: GameMessage,
    src: SocketAddr,
) {
    let disconnect_msg = match serialize_message(&message) {
        Some(bytes) => bytes,
        None => {
            display_error("Failed to serialize disconnect message.");
            return;
        }
    };

    let broadcast_msg = GameMessage {
        message_type: MessageType::ServerInfo,
        sender: "server".to_string(),
        content: MessageContent::ServerInfo {
            server_status: format!("{} has disconnected.", message.sender),
        },
    };

    let broadcast_bytes = match serialize_message(&broadcast_msg) {
        Some(bytes) => bytes,
        None => {
            display_error("Failed to serialize broadcast message.");
            return;
        }
    };

    if let Some(player) = players.remove(&message.sender) {
        if let Err(err) = server_socket.send_to(&disconnect_msg, player.address) {
            display_error(&format!(
                "Failed to send disconnect message to {}: {}",
                player.address, err
            ));
        }

        broadcast_message(
            server_socket,
            players,
            broadcast_bytes,
            Some(&src.to_string()),
            false, 
        );

        display_info(&format!("Player {} has been disconnected.", message.sender));
    } else {
        display_error(&format!(
            "Player {} not found in the connected players list.",
            message.sender
        ));
    }
}

fn handle_player_action(message: GameMessage, src: SocketAddr) {
    println!("Player action from {}: {:?}", src, message.content);
}

fn handle_game_update(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    message: GameMessage,
) {
    println!("message-server {:#?}", message);
    let update_msg = GameMessage {
        message_type: MessageType::GameUpdate,
        sender: message.sender.to_string(),
        content: message.content.clone(),
    };

    let msg_json = match serialize_message(&update_msg) {
        Some(bytes) => bytes,
        None => return,
    };

    broadcast_message(server_socket, &players, msg_json, None, true);
}
