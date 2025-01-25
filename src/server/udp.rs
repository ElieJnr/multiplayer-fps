use super::handlers::handle_message;
use crate::{
    client::{player::Players, udp::client_udp},
    common::{constant::*, protocol::*},
    graphics::resources::PlayerCountState,
    utils::{logger::*, server_utils::*, utils::*},
};
use bevy::math::bool;
use std::{
    io::{self, Write},
    net::UdpSocket,
};

pub fn run_socket() {
    let mut state: PlayerCountState = PlayerCountState::default();
    let mut players = Players::default();

    download_models();
    
    match get_user_choice() {
        Some(1) => {
            handle_server_mode(&mut players);
        }
        Some(2) => {
            handle_client_mode(&mut state);
        }
        Some(_) | None => display_error("Invalid choice. Please choose 1 (Server) or 2 (Client)."),
    }
}

fn handle_server_mode(players: &mut Players) {
    let player_count = get_min_players_from_user();

    match get_local_ipv4() {
        Some(ip) => {
            if let Some(socket) = create_server_socket(ip, PORT) {
                display_info(&format!("Server is running on {}:{}", ip, PORT));
                server(socket, players, &player_count);
            } else {
                display_error("Failed to create server socket.");
            }
        }
        None => display_error("Unable to determine local IP address."),
    }
}

fn get_min_players_from_user() -> PlayerCount {
    print!("Enter the number of players: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>().ok().filter(|&num| num >= DEFAULT_MIN_PLAYERS) {
        Some(num) => PlayerCount::new(num),
        None => {
            display_error("Number of players is less than the minimum required. Using default value.");
            PlayerCount::new(DEFAULT_MIN_PLAYERS)
        }
    }
}

fn handle_client_mode(state: &mut PlayerCountState) {
    if client_udp(state).is_none() {
        display_error("Failed to create client socket.");
    }
}

pub fn server(
    server_socket: UdpSocket,   
    players: &mut Players,
    player_count: &PlayerCount,
) {
    let mut buf = [0; 1024];

    loop {
        match receive_data_from_socket(&server_socket, &mut buf) {
            Some((data, Some(src))) => match deserialize_message(&data) {
                Some(message) => {
                    handle_message(&server_socket, players, message, src, &player_count)
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
    players: Players,
    message: Vec<u8>,
    exclude_name: Option<&str>,
    is_broadcast: bool,
) {
    for player in players.0.values() {
        if is_broadcast || exclude_name.map_or(true, |name| name != player.name) {
            match server_socket.send_to(&message, player.address) {
                Ok(_) => {}
                Err(err) => display_error(&format!(
                    "Failed to send message to {}: {}",
                    player.name, err
                )),
            }
        }
    }
}
