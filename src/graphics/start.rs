use crate::{
    graphics::network::NetworkConfig,
    graphics::resources::Map,
    graphics::states::GameState,
    graphics::systems::{menu::menu_plugin, setup::setup},
};

use bevy::prelude::*;
use std::net::UdpSocket;

pub fn start(player_name: String, server_address: String, client_socket: UdpSocket) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::Map00)
        .insert_resource(NetworkConfig {
            player_name,
            server_address,
            client_socket,
        })
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu_plugin)
        .run();
}
