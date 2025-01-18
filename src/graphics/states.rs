use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
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
