use bevy::prelude::*;
use std::time::Duration;

use crate::{graphics::resources::PlayerCountState, utils::logger::display_info};

use super::protocol::*;

#[derive(Resource, Default)]
pub struct NetworkTimer(Timer);

// Plugin pour gérer la communication réseau
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NetworkTimer(Timer::new(Duration::from_millis(100), TimerMode::Repeating)))
           .add_systems(Update, check_network_messages);
    }
}

// Système pour vérifier les messages réseau
fn check_network_messages(
    time: Res<Time>,
    mut timer: ResMut<NetworkTimer>,
    network_config: Res<NetworkConfig>,
    mut state: ResMut<PlayerCountState>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut buffer = [0; 1024];
        if let Some((data, _)) = receive_data_from_socket(&network_config.client_socket, &mut buffer) {
            if let Some(game_message) = deserialize_message(&data) {
                match game_message.message_type {
                    MessageType::StartGame => {
                        if let MessageContent::StartGame { msg } = game_message.content {
                            state.has_enough_players = true;
                            display_info(&msg);
                        }
                    }
                    MessageType::WaitForPlayers => {
                        if let MessageContent::WaitForPlayers { msg } = game_message.content {
                            state.has_enough_players = false;
                            display_info(&msg);
                        }
                    }
                    // Gérer les autres types de messages si nécessaire
                    _ => {}
                }
            }
        }
    }
}