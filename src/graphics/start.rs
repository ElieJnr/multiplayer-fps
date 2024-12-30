use std::{collections::HashMap, net::UdpSocket, sync::Arc};

use super::{
    map::MazePlugin,
    resources::{PlayerCountState, SoundPlugin},
};
use crate::{
    client::player::*,
    common::protocol::NetworkConfig,
    graphics::{
        resources::Map,
        states::GameState,
        systems::{menu::menu_plugin, setup::setup},
    },
};
use bevy::prelude::*;

pub fn start(
    player_name: String,
    server_address: String,
    client_socket: Arc<UdpSocket>,
    player_count_state: PlayerCountState,
) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::Map00)
        .insert_resource(NetworkConfig {
            player_name,
            server_address,
            client_socket,
        })
        .insert_resource(Players(HashMap::new()))
        .insert_resource(player_count_state)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu_plugin)
        .add_plugins(MazePlugin)
        .add_plugins(SoundPlugin)
        .run();
}
