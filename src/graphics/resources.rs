use bevy::{
    app::{App, Plugin},
    asset::AssetServer,
    audio::{AudioBundle,  AudioSource, PlaybackSettings},
    prelude::{
        Commands, Component, OnEnter, OnExit,  Res, Resource
    },
};

use super::{
    states::GameState,
    systems::menu::despawn_menu,
};

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

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu_track)
            .add_systems(OnEnter(GameState::Waitting), setup_waitting_track)
            .add_systems(OnExit(GameState::Menu), despawn_menu::<MenuTrack>)
            .add_systems(OnExit(GameState::Waitting), despawn_menu::<WaittingTrack>);
    }
}

#[derive(Component)]
struct MenuTrack;

#[derive(Component)]
struct WaittingTrack;

fn setup_menu_track(mut commands: Commands, asset_server: Res<AssetServer>) {
    let track_1 = asset_server.load::<AudioSource>("sounds/menu.ogg");
    commands.spawn((
        AudioBundle {
            source: track_1,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(1.0),
                ..Default::default()
            },
        },
        MenuTrack,
    ));
}

fn setup_waitting_track(mut commands: Commands, asset_server: Res<AssetServer>) {
    let track_2 = asset_server.load::<AudioSource>("sounds/game.ogg");
    commands.spawn((
        AudioBundle {
            source: track_2,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(1.0),
                ..Default::default()
            },
        },
        WaittingTrack,
    ));
}
