use bevy::prelude::*;

#[derive(Debug, Resource, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Map {
    #[default]
    Map00,
    // Map01,
    // Map02,
}

#[derive(Resource)]
pub struct PlayerCountState {
    pub has_enough_players: bool,
}

impl Default for PlayerCountState {
    fn default() -> Self {
        Self {
            has_enough_players: false,
        }
    }
}