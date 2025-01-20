use std::{collections::HashMap, net::UdpSocket, sync::Arc};

use super::{
    map::MazePlugin,
    resources::PlayerCountState,
    systems::{setup::minimap_setup, waitting_page::WaittingRoomPlugin},
};
use crate::{
    client::player::*,
    common::{protocol::NetworkConfig, sync::NetworkPlugin},
    graphics::{resources::Map, states::GameState, systems::menu::menu_plugin},
    player::player::PlayerPlugin,
    maze::{maze::PosStruct, minimap::minimap::{load_minimap_textures, read_maze}},
};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, render::settings::WgpuSettings};

pub fn start(
    player_name: String,
    server_address: String,
    client_socket: Arc<UdpSocket>,
    player_count_state: PlayerCountState,
    initial_position:PosStruct
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
        .init_resource::<PlayerCountState>()
        .insert_resource(player_count_state)
        .insert_resource(initial_position)
        .insert_resource(MyWgpuSettings::new())
        .init_state::<GameState>()
        .add_systems(Startup, (minimap_setup, load_minimap_textures, read_maze))
        .add_plugins(menu_plugin)
        .add_plugins(MazePlugin)
        // .add_plugins(SoundPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(NetworkPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WaittingRoomPlugin);

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

