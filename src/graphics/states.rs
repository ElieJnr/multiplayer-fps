use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Resource)]
pub enum GameState {
    #[default]
    Menu,
    Waitting,
    Game,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Disabled,
    Options,
}
