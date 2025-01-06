use std::{collections::HashMap, net::UdpSocket, sync::Arc};

use super::{map::MazePlugin, resources::PlayerCountState};
use crate::{
    client::player::*,
    common::{protocol::NetworkConfig, sync::NetworkPlugin},
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
    let mut app = App::new();
    
    // Configuration minimale pour les fenêtres
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Game - {}", player_name),
            resolution: (800., 600.).into(),
            ..default()
        }),
        ..default()
    }));

    // Configurations des ressources
    app.insert_resource(Map::Map00)
       .insert_resource(NetworkConfig {
           player_name,
           server_address,
           client_socket,
       })
       .insert_resource(Players(HashMap::new()))
       .insert_resource(player_count_state)
       .init_state::<GameState>();

    // Systèmes principaux
    app.add_systems(Startup, setup)
       .add_plugins(menu_plugin)
       .add_plugins(MazePlugin)
       .add_plugins(NetworkPlugin);

    // Configuration des performances
    /* app.insert_resource(bevy::winit::WinitSettings {
        focused_mode: UpdateMode::Reactive { 
            wait: (), 
            react_to_device_events: (), 
            react_to_user_events: (), 
            react_to_window_events: () 
        },
        unfocused_mode: UpdateMode::ReactiveLowPower {
            wait: (), 
        },
        ..default()
    }); */


    app.run();
}