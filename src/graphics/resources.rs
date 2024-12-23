use bevy::prelude::*;


#[derive(Debug, Resource, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Map {
    #[default]
    Map00,
    // Map01,
    // Map02,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct PlayerCountState {
    pub player_count: usize,
    pub has_enough_players: bool,
}

impl PlayerCountState {
    pub fn new() -> Self {
        PlayerCountState {
            player_count: 0,
            has_enough_players: false,
        }
    }
}
