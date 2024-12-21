use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    client::player::{add_player, Player}, common::{constant::{get_global_socket, get_server_address}, protocol::*}, graphics::start::start, server::udp::broadcast_message, utils::logger::*
};

pub fn handle_message(
    server_socket: &UdpSocket,
    players: &mut HashMap<String, Player>,
    message: GameMessage,
    src: SocketAddr,
) {
    match message.message_type {
        MessageType::NewConnection => handle_new_connection(server_socket, players, message, src),
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
) {
    add_player(players, message.sender.clone(), src);

    let player_name = message.sender.clone();

    let msg = GameMessage {
        message_type: MessageType::NewConnection,
        sender: "server".to_string(),
        content: MessageContent::NewConnection {
            name: message.sender,
        },
    };

    let msg_json = match serialize_message(&msg) {
        Some(bytes) => bytes,
        None => return,
    };

    if let Some(server_address) = get_server_address() {
        let client_socket = get_global_socket().unwrap_or_else(|| {
            UdpSocket::bind("0.0.0.0:0").expect("Failed to create a dummy socket")
        });
        start(player_name, server_address, client_socket);
    } else {
        display_warning("Server address is not set yet.");
    }
    broadcast_message(server_socket, &players, msg_json, Some(&src.to_string()));
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

        broadcast_message(server_socket, players, broadcast_bytes, Some(&src.to_string()));

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
    let update_msg = GameMessage {
        message_type: MessageType::GameUpdate,
        sender: "server".to_string(),
        content: message.content.clone(),
    };

    let msg_json = match serialize_message(&update_msg) {
        Some(bytes) => bytes,
        None => return,
    };

    broadcast_message(server_socket, &players, msg_json, None);
}
