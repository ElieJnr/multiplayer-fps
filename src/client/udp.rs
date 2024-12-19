use serde_json;
use std::io::{self, Write};
use std::{io::stdin, net::UdpSocket};

use crate::utils::data_handling::serialize_message;
use crate::utils::model::{GameMessage, MessageContent, MessageType};

pub fn client_udp() -> Option<()> {
    let (name, ip) = get_user_input()?;
    let socket = create_socket()?;
    connect_to_server(&socket, &ip)?;
    send_new_connection(&socket, &name)?;
    receive_game_update(&socket);
    Some(())
}

fn get_user_input() -> Option<(String, String)> {
    let mut name = String::new();
    let mut ip = String::new();

    print!("Enter The Server IP Address: ");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut ip).expect("failed to read the IP");
    print!("Enter Your Name: ");
    io::stdout().flush().unwrap();
    stdin()
        .read_line(&mut name)
        .expect("failed to read the name");

    ip = ip.trim().to_string();
    name = name.trim().to_string();

    Some((name, ip))
}

fn create_socket() -> Option<UdpSocket> {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => Some(socket),
        Err(err) => {
            eprintln!("Failed to bind client socket: {}", err);
            None
        }
    }
}

fn connect_to_server(socket: &UdpSocket, ip: &str) -> Option<()> {
    if let Err(err) = socket.connect(ip) {
        eprintln!("Failed to connect to server {}: {}", ip, err);
        None
    } else {
        println!("Connected to server at {}", ip);
        Some(())
    }
}

fn send_new_connection(socket: &UdpSocket, name: &str) -> Option<()> {
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
        eprintln!("Failed to send message: {}", err);
        None
    } else {
        Some(())
    }
}

fn receive_game_update(socket: &UdpSocket) {
    let mut buffer = [0; 1024];

    loop {
        match socket.recv(&mut buffer) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buffer[..size]);
                let game_message: GameMessage = match serde_json::from_str(&response) {
                    Ok(msg) => msg,
                    Err(err) => {
                        eprintln!("Error deserializing message: {}", err);
                        continue;
                    }
                };

                match game_message.message_type {
                    MessageType::GameUpdate => {
                        println!("Received game update: {:?}", game_message.content);
                    }
                    MessageType::Disconnect => {
                        handle_disconnect(game_message.content);
                        break;
                    }
                    MessageType::NewConnection => handle_new_connection(game_message.content),
                    MessageType::PlayerAction => {},
                    MessageType::ServerInfo => handle_server_info(game_message.content),
                }
            }
            Err(err) => eprintln!("Failed to receive response: {}", err),
        }
    }
}

fn handle_disconnect(content: MessageContent) {
    if let MessageContent::Disconnect { reason } = content {
        println!("[{}]", reason);
    } else {
        eprintln!("Received invalid content type for NewConnection");
    }
}

fn handle_server_info(content: MessageContent) {
    if let MessageContent::ServerInfo { server_status } = content {
        println!("[{}]", server_status);
    } else {
        eprintln!("Received invalid content type for ServerInfo");
    }
}

fn handle_new_connection(content: MessageContent) {
    if let MessageContent::NewConnection { name } = content {
        println!("[{} has joined the game.]", name);
    } else {
        eprintln!("Received invalid content type for NewConnection");
    }
}
