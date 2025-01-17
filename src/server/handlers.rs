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

use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::Rng;
use bevy::math::vec3;
use bevy::math::Vec3;

pub fn 
handle_message(
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
    let initial_position= choose_place(& ALL_POSITION, & IS_OCCUPED);
    add_player(players, message.sender.clone(), src, initial_position);
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
            players:players.clone(),
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
            players: players.clone()
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

lazy_static! {
    pub static ref ALL_POSITION: Mutex<HashMap<String, Vec<f32>>> = {
        let mut map = HashMap::new();
        map.insert("1".to_string(), vec![20.0, 1.0, 37.0]);
        map.insert("2".to_string(), vec![24.29, 1.0, 25.13]);
        map.insert("3".to_string(), vec![15.84, 1.0, 19.92]);
        map.insert("4".to_string(), vec![25.97, 1.0, 18.76]);
        map.insert("5".to_string(), vec![37.70, 1.0, 21.66]);
        map.insert("6".to_string(), vec![24.14, 1.0, 1.41]);
        map.insert("7".to_string(), vec![37.0, 1.0, 35.0]);
        map.insert("8".to_string(), vec![37.4, 1.0, 1.2]);
        map.insert("9".to_string(), vec![1.27, 1.0, 5.63]);
        map.insert("10".to_string(), vec![1.68, 1.0, 18.91]);
        Mutex::new(map)
    };

    pub static ref IS_OCCUPED: Mutex<HashMap<String, bool>> = {
        let mut map = HashMap::new();
        map.insert("1".to_string(), false);
        map.insert("2".to_string(), false);
        map.insert("3".to_string(), false);
        map.insert("4".to_string(), false);
        map.insert("5".to_string(), false);
        map.insert("6".to_string(), false);
        map.insert("7".to_string(), false);
        map.insert("8".to_string(), false);
        map.insert("9".to_string(), false);
        map.insert("10".to_string(), false);
        Mutex::new(map)
    };
}

pub fn set_is_occupied(key: &str, value: bool) {
    let mut is_occuped = IS_OCCUPED.lock().unwrap();
    if let Some(entry) = is_occuped.get_mut(key) {
        *entry = value;
    }
}

fn choose_place(
    all_position: &Mutex<HashMap<String, Vec<f32>>>, is_occuped: &Mutex<HashMap<String, bool>>
) -> Vec3 {
    let all_position_locked= all_position.lock().unwrap();
    let mut is_occuped_locked=is_occuped.lock().unwrap();
    let mut rng = rand::thread_rng();
    
    // Générer un index aléatoire basé sur la longueur de name_position
    let random_number = rng.gen_range(0..all_position_locked.len());
    
    // Obtenir la clé correspondant à l'index aléatoire
    let random_key = all_position_locked.keys().nth(random_number).unwrap().clone();
    
    // Vérifier si la clé existe dans bool_position
    match is_occuped_locked.get_mut(&random_key) {
        Some(bool_value) => {
            // Vérifier si le lieu a déjà été choisi (i.e., si la valeur booléenne est true)
            if *bool_value {
                // Si déjà choisi, on rappelle la fonction pour essayer de choisir un autre lieu
                return choose_place(all_position, is_occuped);
            } else {
                // Marquer le lieu comme choisi (mettre la valeur à true)
                *bool_value = true;
                
                // Retourner la valeur correspondante de name_position
                let position= all_position_locked.get(&random_key).unwrap().clone();
                return vec3(position[0], position[1], position[2])
            }
        },
        None => {
            // Si la clé n'existe pas dans bool_position, on rappelle la fonction pour essayer encore
            return choose_place(all_position, is_occuped);
        }
    }
}