use std::{net::UdpSocket, sync::Arc};

use super::{
    map::MazePlugin,
    resources::{PlayerCountState, SoundPlugin},
    systems::{setup::minimap_setup, waitting_page::WaittingRoomPlugin},
};
use crate::{
    client::player::*,
    common::{protocol::NetworkConfig, sync::NetworkPlugin},
    graphics::{resources::Map, states::GameState, systems::menu::menu_plugin},
    maze::{
        maze::PosStruct,
        minimap::minimap::{load_minimap_textures, read_maze},
    },
    player::model::PlayerPlugin,
};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::settings::WgpuSettings};

pub struct GameConfig {
    pub player_name: String,
    pub server_address: String,
    pub client_socket: Arc<UdpSocket>,
    pub player_count_state: PlayerCountState,
    pub initial_position: PosStruct,
}

pub fn start(config: GameConfig) {
    let mut app = App::new();

    // Window configuration
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Game - {}", config.player_name),
            resolution: (1200., 1000.).into(),
            ..default()
        }),
        ..default()
    }));

    // Resource initialization
    app.insert_resource(Map::Map00)
        .insert_resource(NetworkConfig {
            player_name: config.player_name,
            server_address: config.server_address,
            client_socket: config.client_socket,
        })
        .init_resource::<Players>()
        .init_resource::<PlayerCountState>()
        .insert_resource(config.player_count_state)
        .insert_resource(config.initial_position)
        .insert_resource(MyWgpuSettings::new())
        .init_state::<GameState>();

    // System setup
    app.add_systems(Startup, (minimap_setup, load_minimap_textures, read_maze));

    // Plugin setup
    app.add_plugins((
        menu_plugin,
        MazePlugin,
        SoundPlugin,
        FrameTimeDiagnosticsPlugin,
        NetworkPlugin,
        PlayerPlugin,
        WaittingRoomPlugin,
    ));

    app.run();
}

#[derive(Resource)]
pub struct MyWgpuSettings(WgpuSettings);

impl MyWgpuSettings {
    fn new() -> Self {
        MyWgpuSettings(WgpuSettings {
            power_preference: bevy::render::settings::PowerPreference::HighPerformance,
            ..default()
        })
    }

    pub fn get_settings(&self) -> &WgpuSettings {
        &self.0
    }
}
