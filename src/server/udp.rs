use crate::{
    client::udp::client_udp, common::protocol::*, utils::{logger::*, server_utils::*, utils::*}
};
use std::{
    collections::HashMap, net::{SocketAddr, UdpSocket}
};

use super::handlers::handle_message;

const PORT: &str = "8080";

pub fn run_socket() {
    let mut client_addresses = HashMap::new();

    let choice = match get_user_choice() {
        Some(choice) => choice,
        None => {
            display_error("Failed to get a valid choice.");
            return;
        }
    };
    clear_screen();

    match choice {
        1 => handle_server_mode(&mut client_addresses),
        2 => handle_client_mode(),
        _ => display_error("Invalid choice. Please choose 1 (Server) or 2 (Client)."),
    }
}

fn handle_server_mode(client_addresses: &mut HashMap<String, SocketAddr>) {
    match get_local_ipv4() {
        Some(ip) => {
            if let Some(socket) = create_server_socket(ip, PORT) {
                display_warning(&format!("Server is running on {}:{}", ip, PORT));
                server(socket, client_addresses);
            } else {
                display_error("Failed to create server socket.");
            }
        }
        None => display_error("Unable to determine local IP address."),
    }
}

fn handle_client_mode() {
    if client_udp().is_none() {
        display_error("Failed to create client socket.");
    }
}

pub fn server(server_socket: UdpSocket, client_addresses: &mut HashMap<String, SocketAddr>) {
    let mut buf = [0; 1024];
    loop {
        match receive_data_from_socket(&server_socket, &mut buf) {
            Some((data, Some(src))) => match deserialize_message(&data) {
                Some(message) => {
                    handle_message(&server_socket, client_addresses, message, src);
                }
                None => display_error("Erreur lors de la désérialisation"),
            },
            Some((_, None)) => display_error("Failed to receive source address"),
            None => display_error("Failed to receive data"),
        }
    }
}

pub fn broadcast_message(
    server_socket: &UdpSocket,
    client_addresses: &HashMap<String, SocketAddr>,
    message: Vec<u8>,
    exclude_addr: Option<SocketAddr>,
) {
    for (_, client_src) in client_addresses.iter() {
        if Some(*client_src) != exclude_addr {
            if let Err(err) = server_socket.send_to(&message, client_src) {
                display_error(&format!("Failed to send message to {}: {}", client_src, err));
            }
        }
    }
}

