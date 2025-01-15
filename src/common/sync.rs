use bevy::prelude::*;
use std::time::Duration;

use crate::{graphics::resources::PlayerCountState, utils::logger::display_info};

use super::protocol::{deserialize_message, receive_data_from_socket, GameMessage, MessageContent, MessageType, NetworkConfig};

#[derive(Resource)]
pub struct NetworkTimer {
    timer: Timer,
    last_check: f32,
}

impl Default for NetworkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(50), TimerMode::Repeating), // Augmenté à 500ms
            last_check: 0.0,
        }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkTimer>()
           .add_systems(Update, check_network_messages);
    }
}

fn check_network_messages(
    time: Res<Time>,
    mut timer: ResMut<NetworkTimer>,
    network_config: Res<NetworkConfig>,
    mut state: ResMut<PlayerCountState>,
) {
    if time.elapsed_seconds() - timer.last_check < 0.1 {
        return;
    }
    timer.last_check = time.elapsed_seconds();

    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    // Configure le socket en mode non-bloquant
    if let Ok(socket) = network_config.client_socket.try_clone() {
        let _ = socket.set_nonblocking(true);
        
        let mut buffer = [0; 1024];
        match receive_data_from_socket(&socket, &mut buffer) {
            Some((data, _)) => {
                if let Some(game_message) = deserialize_message(&data) {
                    handle_game_message(game_message, &mut state);
                }
            }
            None => {} // Pas de message disponible, continue normalement
        }
    }
}

// Sépare la logique de traitement des messages
fn handle_game_message(message: GameMessage, state: &mut PlayerCountState) {
    match message.message_type {
        MessageType::StartGame => {
            if let MessageContent::StartGame { msg } = message.content {
                state.has_enough_players = true;
                display_info(&msg);
            }
        }
        MessageType::WaitForPlayers => {
            if let MessageContent::WaitForPlayers { msg } = message.content {
                state.has_enough_players = false;
                display_info(&msg);
            }
        }
        _ => {}
    }
}