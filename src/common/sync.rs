use bevy::prelude::*;
use std::collections::VecDeque;

use crate::{graphics::resources::PlayerCountState, utils::logger::display_info};

use super::protocol::{
    deserialize_message, receive_data_from_socket, GameMessage, MessageContent, MessageType,
    NetworkConfig,
};

#[derive(Resource)]
pub struct NetworkTimer {
    // timer: Timer,
    last_check: f32,
}

impl Default for NetworkTimer {
    fn default() -> Self {
        Self { last_check: 0.0 }
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkTimer>()
            .init_resource::<NetworkGameUpdate>()
            .init_resource::<NetworkMessages>()
            .add_systems(Update, check_network_messages);
    }
}

#[derive(Resource)]
pub struct NetworkMessages(pub VecDeque<GameMessage>);

impl Default for NetworkMessages {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

#[derive(Resource)]
pub struct NetworkGameUpdate(pub VecDeque<GameMessage>);

impl Default for NetworkGameUpdate {
    fn default() -> Self {
        Self(VecDeque::new())
    }
}

fn check_network_messages(
    time: Res<Time>,
    mut timer: ResMut<NetworkTimer>,
    network_config: Res<NetworkConfig>,
    mut state: ResMut<PlayerCountState>,
    mut network_messages: ResMut<NetworkMessages>,
    mut network_update: ResMut<NetworkGameUpdate>,
) {
    if time.elapsed_seconds() - timer.last_check < 0.016 {
        return;
    }

    timer.last_check = time.elapsed_seconds();

    if let Ok(socket) = network_config.client_socket.try_clone() {
        let _ = socket.set_nonblocking(true);
        let mut buffer = [0; 1024];

        match receive_data_from_socket(&socket, &mut buffer) {
            Some((data, _)) => {
                if let Some(game_message) = deserialize_message(&data) {
                    match game_message.message_type {
                        MessageType::DecreaseLife
                        // | MessageType::PlayerAction
                        // | MessageType::SyncPlayers 
                        =>
                         {
                            network_messages.0.push_back(game_message);
                        }
                        MessageType::GameUpdate => {
                            network_update.0.push_back(game_message);
                        }
                        _ => handle_game_message(game_message, &mut state),
                    }
                }
            }
            None => {}
        }
    }
}

fn handle_game_message(message: GameMessage, state: &mut PlayerCountState) {
    match message.message_type {
        MessageType::StartGame => {
            if let MessageContent::StartGame { msg, .. } = message.content {
                state.has_enough_players = true;
                display_info(&msg);
            }
        }
        MessageType::WaitForPlayers => {
            if let MessageContent::WaitForPlayers { msg, .. } = message.content {
                state.has_enough_players = false;
                display_info(&msg);
            }
        }
        _ => {}
    }
}
