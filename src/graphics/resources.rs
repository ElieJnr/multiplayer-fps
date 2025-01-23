use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    audio::{AudioBundle, AudioSource, PlaybackSettings},
    input::ButtonInput,
    prelude::{Commands, Component, KeyCode, OnEnter, OnExit, Res, Resource},
};

use super::{states::GameState, systems::menu::despawn_menu};

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
            .add_systems(OnExit(GameState::Waitting), despawn_menu::<WaittingTrack>)
            .add_systems(Update, setup_gameplay_track);
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

fn setup_gameplay_track(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let gunshot = asset_server.load::<AudioSource>("sounds/gunshot.ogg");
    let walk = asset_server.load::<AudioSource>("sounds/walk.ogg");

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.spawn((AudioBundle {
            source: gunshot,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                volume: bevy::audio::Volume::new(1.0),
                ..Default::default()
            },
        },));
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        commands.spawn((AudioBundle {
            source: walk,
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Once,
                volume: bevy::audio::Volume::new(1.0),
                ..Default::default()
            },
        },));
    }
}
