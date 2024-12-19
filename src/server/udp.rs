use crate::{client::{graphics::start, udp::client_udp}, utils::{data_handling::serialize_message, model::{GameMessage, MessageContent, MessageType}}};
use serde_json;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

fn get_local_ipv4() -> Option<Ipv4Addr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    if let Ok(local_addr) = socket.local_addr() {
        if let IpAddr::V4(ipv4) = local_addr.ip() {
            return Some(ipv4);
        }
    }
    None
}

fn create_server_socket(ip: Ipv4Addr) -> Option<UdpSocket> {
    match UdpSocket::bind(format!("{}:{}", ip.to_string(), "8080")) {
        Ok(socket) => Some(socket),
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}

pub fn run_socket() {
    let mut choice = String::new();
    let mut client_addresses = HashMap::new();
    println!("Your choice please: \n1-Server \n2-Client");

    let _ = stdout().flush();
    stdin()
        .read_line(&mut choice)
        .expect("not a correct number");

    let choice_int = match choice.trim().parse::<u32>() {
        Ok(choice_int) => choice_int,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match choice_int {
        1 => match get_local_ipv4() {
            Some(ip) => {
                if let Some(socket) = create_server_socket(ip) {
                    println!("Server is running on {}:8080", ip);
                    server(socket,&mut client_addresses);
                }
            }
            None => eprintln!("Unable to determine local IP address."),
        },
        2 => {
            if client_udp().is_none() {
                eprintln!("Failed to create client socket");
            }

        }
        _ => eprintln!("make a choice between 1 and 2"),
    }
}

pub fn server(server_socket: UdpSocket, client_addresses: &mut HashMap<String, SocketAddr>) {
    let mut buf = [0; 1024];
    loop {
        match receive_message(&server_socket, &mut buf) {
            Some((size, src)) => {
                match deserialize_message(&buf[..size]) {
                    Some(message) => {
                        handle_message(&server_socket, client_addresses, message, src);
                    }
                    None => eprintln!("Erreur lors de la désérialisation"),
                }
            }
            None => eprintln!("Failed to receive data"),
        }
    }
}

fn receive_message(server_socket: &UdpSocket, buf: &mut [u8]) -> Option<(usize, SocketAddr)> {
    match server_socket.recv_from(buf) {
        Ok((size, src)) => Some((size, src)),
        Err(err) => {
            eprintln!("Failed to receive data: {}", err);
            None
        }
    }
}

fn deserialize_message(data: &[u8]) -> Option<GameMessage> {
    match serde_json::from_slice(data) {
        Ok(message) => Some(message),
        Err(e) => {
            eprintln!("Erreur lors de la désérialisation: {}", e);
            None
        }
    }
}

fn handle_message(server_socket: &UdpSocket, client_addresses: &mut HashMap<String, SocketAddr>, message: GameMessage, src: SocketAddr) {
    match message.message_type {
        MessageType::NewConnection => handle_new_connection(server_socket, client_addresses, message, src),
        MessageType::PlayerAction => handle_player_action(message, src),
        MessageType::GameUpdate => handle_game_update(server_socket, client_addresses, message),
        _ => eprintln!("Unhandled message type: {:?}", message.message_type),
    }
}

fn handle_new_connection(server_socket: &UdpSocket, client_addresses: &mut HashMap<String, SocketAddr>, message: GameMessage, src: SocketAddr) {
    client_addresses.insert(message.sender.clone(), src);
    println!("{} has joined", message.sender);

    let msg = GameMessage {
        message_type: MessageType::NewConnection,
        sender: "server".to_string(),
        content: MessageContent::NewConnection { name: message.sender},
    };

    let msg_json = match serialize_message(&msg) {
        Some(bytes) => bytes,
        None => return,
    };
    // appel de la fonction de graphics (UI client)
    start();
    
    broadcast_message(server_socket, client_addresses, msg_json, Some(src));
}

fn handle_player_action(message: GameMessage, src: SocketAddr) {
    println!("Player action from {}: {:?}", src, message.content);
}

fn handle_game_update(server_socket: &UdpSocket, client_addresses: &HashMap<String, SocketAddr>, message: GameMessage) {
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

fn broadcast_message(server_socket: &UdpSocket, client_addresses: &HashMap<String, SocketAddr>, message: Vec<u8>, exclude_addr: Option<SocketAddr>) {
    for (_, client_src) in client_addresses.iter() {
        if Some(*client_src) != exclude_addr {
            if let Err(err) = server_socket.send_to(&message, client_src) {
                eprintln!("Failed to send message to {}: {}", client_src, err);
            }
        }
    }
}