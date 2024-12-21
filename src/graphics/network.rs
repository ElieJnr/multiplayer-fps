use std::net::UdpSocket;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct NetworkConfig {
    pub player_name: String,
    pub server_address: String,
    pub client_socket: UdpSocket,
}
