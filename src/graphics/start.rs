use std::collections::HashMap;

use super::map::MazePlugin;
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

pub fn start(config: &NetworkConfig) {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::Map00)
        .insert_resource(config.clone())
        .insert_resource(Players(HashMap::new()))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu_plugin)
        .add_plugins(MazePlugin)
        .run();
}
