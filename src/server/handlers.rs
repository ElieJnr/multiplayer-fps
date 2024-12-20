use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    client::graphics::start,
    common::protocol::*,
    server::udp::broadcast_message,
    utils::{logger::*, server_utils::get_server_address},
};

pub fn handle_message(
    server_socket: &UdpSocket,
    client_addresses: &mut HashMap<String, SocketAddr>,
    message: GameMessage,
    src: SocketAddr,
) {
    match message.message_type {
        MessageType::NewConnection => {
            handle_new_connection(server_socket, client_addresses, message, src)
        }
        MessageType::PlayerAction => handle_player_action(message, src),
        MessageType::GameUpdate => handle_game_update(server_socket, client_addresses, message),
        MessageType::Disconnect => handle_disconnect(server_socket, client_addresses, message, src),
        _ => display_error(&format!(
            "Unhandled message type: {:?}",
            message.message_type
        )),
    }
}

fn handle_new_connection(
    server_socket: &UdpSocket,
    client_addresses: &mut HashMap<String, SocketAddr>,
    message: GameMessage,
    src: SocketAddr,
) {
    client_addresses.insert(message.sender.clone(), src);
    display_info(&format!("{} has joined", message.sender));

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
        start(player_name, server_address);
    } else {
        display_warning("Server address is not set yet.");
    }

    broadcast_message(server_socket, client_addresses, msg_json, Some(src));
}

fn handle_disconnect(
    server_socket: &UdpSocket,
    client_addresses: &mut HashMap<String, SocketAddr>,
    message: GameMessage,
    src: SocketAddr,
) {
    let msg_json = match serialize_message(&message) {
        Some(bytes) => bytes,
        None => return,
    };

    let msg = GameMessage {
        message_type: MessageType::ServerInfo,
        sender: "server".to_string(),
        content: MessageContent::ServerInfo {
            server_status: format!("{} is disconnected", message.sender),
        },
    };

    let info_msg = match serialize_message(&msg) {
        Some(bytes) => bytes,
        None => return,
    };

    if let Some(client_addr) = client_addresses.remove(&message.sender) {
        if let Err(err) = server_socket.send_to(&msg_json, client_addr) {
            display_error(&format!("Failed to send disconnect message to {}: {}", client_addr, err));
        } else {
            broadcast_message(server_socket, client_addresses, info_msg, Some(src));
        }
    } else {
        display_error(&format!("Failed to find address for {}", message.sender));
    }
}

fn handle_player_action(message: GameMessage, src: SocketAddr) {
    println!("Player action from {}: {:?}", src, message.content);
}

fn handle_game_update(
    server_socket: &UdpSocket,
    client_addresses: &HashMap<String, SocketAddr>,
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

    broadcast_message(server_socket, client_addresses, msg_json, None);
}
