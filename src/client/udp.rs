use crate::common::protocol::*;
use crate::graphics::resources::PlayerCountState;
use crate::utils::logger::*;
use crate::utils::utils::*;
use std::net::UdpSocket;
use std::sync::Arc;

use super::handlers::*;
pub fn client_udp(state: &mut PlayerCountState) -> Option<NetworkConfig> {
    let network_config = initialize_network_config()?;

    connect_to_server(&network_config)?;
    send_new_connection(&network_config)?;
    receive_server_message(&network_config, state);
    Some(network_config)
}

fn initialize_network_config() -> Option<NetworkConfig> {
    let (name, ip) = get_user_input()?;
    let socket = create_socket()?;
    Some(NetworkConfig {
        player_name: name,
        server_address: ip,
        client_socket: Arc::new(socket),
    })
}

pub fn create_socket() -> Option<UdpSocket> {
    match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => Some(socket),
        Err(err) => {
            eprintln!("Failed to bind socket: {}", err);
            None
        }
    }
}

fn connect_to_server(config: &NetworkConfig) -> Option<()> {
    if let Err(err) = config.client_socket.connect(&config.server_address) {
        let reason = format!(
            "Failed to connect to server {}: {}",
            config.server_address, err
        );
        send_disconnect_message(config, "server", &reason);
        display_error(&reason);
        None
    } else {
        display_info(&format!("Connected to server at {}", config.server_address));
        Some(())
    }
}

fn receive_server_message(config: &NetworkConfig, state: &mut PlayerCountState) {
    let mut buffer = [0; 1024];
    let mut error_count = 0;

    loop {
        match receive_data_from_socket(&config.client_socket, &mut buffer) {
            Some((data, _)) => {
                if let Some(game_message) = deserialize_message(&data) {
                    // display_info(&format!("CHECK {:#?}", game_message));
                    error_count = 0;

                    if process_game_message(game_message, config, state) {
                        break;
                    }
                } else {
                    display_error("Failed to deserialize message.");
                }
            }
            None => {
                display_error("Failed to receive data.");
                error_count += 1;
            }
        }
        if error_count >= 5 {
            display_warning("Too many errors. Disconnecting the player.");
            handle_disconnect(MessageContent::Disconnect {
                reason: "Failed to receive data".to_string(),
            });
            break;
        }
    }
}

fn process_game_message(
    game_message: GameMessage,
    config: &NetworkConfig,
    state: &mut PlayerCountState,
) -> bool {
    match game_message.message_type {
        MessageType::GameUpdate => {
            // println!("Received game update: {:?}", game_message.content);
        }
        MessageType::Disconnect => {
            handle_disconnect(game_message.content);
            return true;
        }
        MessageType::NewConnection => handle_new_connection(game_message.content),
        MessageType::PlayerAction => {}
        MessageType::ServerInfo => handle_server_info(game_message.content),
        MessageType::WaitForPlayers => handle_waiting(game_message.content, config, state),
        MessageType::StartGame => handle_start(game_message.content, config, state),
    }
    false
}
