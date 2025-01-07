use std::{collections::HashMap, net::UdpSocket, sync::Arc};

use super::{map::MazePlugin, resources::PlayerCountState, systems::setup::minimap_setup};
use crate::{
    client::player::*,
    common::{protocol::NetworkConfig, sync::NetworkPlugin},
    graphics::{
        resources::Map,
        states::GameState,
        systems::menu::menu_plugin,
    },
};
use bevy::prelude::*;

pub fn start(
    player_name: String,
    server_address: String,
    client_socket: Arc<UdpSocket>,
    player_count_state: PlayerCountState,
) {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Game - {}", player_name),
            resolution: (1200., 1000.).into(),
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }));

    app.insert_resource(Map::Map00)
       .insert_resource(NetworkConfig {
           player_name,
           server_address,
           client_socket,
       })
       .insert_resource(Players(HashMap::new()))
       .insert_resource(player_count_state)
       .init_state::<GameState>();

    app.add_systems(Startup, minimap_setup)
       .add_plugins(menu_plugin)
       .add_plugins(MazePlugin)
       .add_plugins(NetworkPlugin);

    app.run();
}