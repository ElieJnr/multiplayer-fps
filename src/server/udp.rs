use super::handlers::handle_message;
use crate::{
    client::{player::Player, udp::client_udp},
    common::{constant::*, protocol::*},
    graphics::resources::PlayerCountState,
    utils::{logger::*, server_utils::*, utils::*},
};
use std::{collections::HashMap, net::UdpSocket};
use bevy::{math::bool, prelude::Has};
use rand::Rng;

pub fn run_socket() {
    let mut state = PlayerCountState::default();
    let mut players: HashMap<String, Player> = HashMap::new();

    let mut name_position:HashMap<String, Vec<f32>>=HashMap::new();
    name_position.insert("1".to_string(),vec![20.0, 1.0, 37.0]);
    name_position.insert("2".to_string(),vec![24.29, 1.0, 25.13]);
    name_position.insert("3".to_string(),vec![15.84, 1.0, 19.92]);
    name_position.insert("4".to_string(),vec![25.97, 1.0, 18.76]);
    name_position.insert("5".to_string(),vec![37.70, 1.0, 21.66]);
    name_position.insert("6".to_string(),vec![24.14, 1.0, 1.41]);
    name_position.insert("7".to_string(),vec![37.0, 1.0, 35.0]);
    name_position.insert("8".to_string(),vec![37.4, 1.0, 1.2]);
    name_position.insert("9".to_string(),vec![1.27, 1.0, 5.63]);
    name_position.insert("10".to_string(),vec![1.68, 1.0, 18.91]);

    let mut bool_position: HashMap<String,bool>=HashMap::new();
    bool_position.insert("1".to_string(),false);
    bool_position.insert("2".to_string(),false);
    bool_position.insert("3".to_string(),false);
    bool_position.insert("4".to_string(),false);
    bool_position.insert("5".to_string(),false);
    bool_position.insert("6".to_string(),false);
    bool_position.insert("7".to_string(),false);
    bool_position.insert("8".to_string(),false);
    bool_position.insert("9".to_string(),false);
    bool_position.insert("10".to_string(),false);


    match get_user_choice() {
        Some(1) => handle_server_mode(&mut players, &mut state),
        Some(2) => handle_client_mode(&mut state),
        Some(_) | None => display_error("Invalid choice. Please choose 1 (Server) or 2 (Client)."),
    }
}

fn choose_place(
    name_position: &mut HashMap<String, Vec<f32>>,
    bool_position: &mut HashMap<String, Vec<bool>>,
) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    
    // Generate a random index based on the length of name_position
    let random_number = rng.gen_range(0..name_position.len());
    
    // Get the key corresponding to the random index
    let random_key = name_position.keys().nth(random_number).unwrap().clone();
    
    // Check if the key exists in bool_position
    match bool_position.get_mut(&random_key) {
        Some(bool_vec) => {
            // Check if the place has already been chosen (i.e., bool value is true)
            if bool_vec.contains(&true) {
                // If already chosen, recurse and try again
                return choose_place(name_position, bool_position);
            } else {
                // Mark the place as chosen (set to true)
                bool_vec.push(true);
                
                // Return the corresponding value from name_position
                return name_position.get(&random_key).unwrap().clone();
            }
        },
        None => {
            // If the key doesn't exist in bool_position, recurse and try again
            return choose_place(name_position, bool_position);
        }
    }
}

fn handle_server_mode(players: &mut HashMap<String, Player>, state: &mut PlayerCountState) {
    match get_local_ipv4() {
        Some(ip) => {
            if let Some(socket) = create_server_socket(ip, PORT) {
                display_info(&format!("Server is running on {}:{}", ip, PORT));
                server(socket, players, state);
            } else {
                display_error("Failed to create server socket.");
            }
        }
        None => display_error("Unable to determine local IP address."),
    }
}

fn handle_client_mode(state: &mut PlayerCountState) {
    if client_udp(state).is_none() {
        display_error("Failed to create client socket.");
    }
}

pub fn server(
    server_socket: UdpSocket,
    players: &mut HashMap<String, Player>,
    state: &mut PlayerCountState,
) {
    let mut buf = [0; 1024];

    loop {
        match receive_data_from_socket(&server_socket, &mut buf) {
            Some((data, Some(src))) => match deserialize_message(&data) {
                Some(message) => handle_message(&server_socket, players, message, src, state),
                None => display_error("Erreur lors de la désérialisation"),
            },
            Some((_, None)) => display_error("Failed to receive source address"),
            None => display_error("Failed to receive data"),
        }
    }
}

pub fn broadcast_message(
    server_socket: &UdpSocket,
    players: &HashMap<String, Player>,
    message: Vec<u8>,
    exclude_name: Option<&str>,
    is_broadcast: bool,
) {
    for player in players.values() {
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
