use std::net::UdpSocket;

use crate::common::constant::*;
use crate::common::protocol::*;
use crate::utils::logger::*;
use crate::utils::utils::*;

use super::handlers::*;

pub fn client_udp() -> Option<()> {
    let (name, ip) = get_user_input()?;
    let socket = create_socket()?;

    connect_to_server(&socket, &ip)?;
    send_new_connection(&socket, &name)?;
    receive_server_message(&socket);
    Some(())
}

pub fn create_socket() -> Option<UdpSocket> {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            set_global_socket(socket.try_clone().ok()?);
            Some(socket)
        }
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}

fn connect_to_server(socket: &UdpSocket, ip: &str) -> Option<()> {
    if let Err(err) = socket.connect(ip) {
        let reason = format!("Failed to connect to server {}: {}", ip, err);
        send_disconnect_message(socket, "server", &reason);
        display_error(&reason);
        None
    } else {
        display_info(&format!("Connected to server at {}", ip));
        Some(())
    }
}

fn receive_server_message(socket: &UdpSocket) {
    let mut buffer = [0; 1024];
    loop {
        match receive_data_from_socket(socket, &mut buffer) {
            Some((data, _)) => {
                if let Some(game_message) = deserialize_message(&data) {
                    if process_game_message(game_message) {
                        break;
                    }
                } else {
                    display_error("Failed to deserialize message.");
                }
            }
            None => display_error("Failed to receive data."),
        }
    }
}

fn process_game_message(game_message: GameMessage)-> bool {
    match game_message.message_type {
        MessageType::GameUpdate => {
            println!("Received game update: {:?}", game_message.content);
        }
        MessageType::Disconnect => {
            handle_disconnect(game_message.content);
            return true;
        }
        MessageType::NewConnection => handle_new_connection(game_message.content),
        MessageType::PlayerAction => {}
        MessageType::ServerInfo => handle_server_info(game_message.content),
    }
    false
}

