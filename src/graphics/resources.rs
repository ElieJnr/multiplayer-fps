use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Handle},
    audio::{AudioBundle, AudioSink, AudioSinkPlayback, AudioSource, PlaybackSettings},
    log::info,
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, IntoSystemConfigs, OnEnter, OnExit,
        Query, Res, ResMut, Resource, With,
    },
    time::Time,
};

use super::states::GameState;

// use bevy::audio::{AudioBundle, AudioSink, PlaybackSettings};

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
        app.insert_resource(CurrentTrack::None)
            .add_systems(
                OnEnter(GameState::Menu),
                (setup_sound, setup_menu_sound).chain(),
            )
            .add_systems(OnEnter(GameState::Game), setup_game_sound)
            .add_systems(Update, (fade_in, fade_out))
            .add_systems(OnExit(GameState::Menu), fade_out_menu)
            .add_systems(OnExit(GameState::Game), fade_out_game);
    }
}

#[derive(Component)]
pub struct FadeIn;

#[derive(Component)]
pub struct FadeOut;

#[derive(Resource)]
struct SoundtrackPlayer {
    menu_track: Handle<AudioSource>,
    game_track: Handle<AudioSource>,
}

impl SoundtrackPlayer {
    fn new(menu_track: Handle<AudioSource>, game_track: Handle<AudioSource>) -> Self {
        Self {
            menu_track,
            game_track,
        }
    }
}

#[derive(Resource)]
enum CurrentTrack {
    Menu,
    Game,
    None,
}

fn setup_menu_sound(
    mut commands: Commands,
    soundtrack_player: Res<SoundtrackPlayer>,
    mut current_track: ResMut<CurrentTrack>,
) {
    commands.spawn((
        AudioBundle {
            source: soundtrack_player.menu_track.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::ZERO,
                ..Default::default()
            },
            ..Default::default()
        },
        FadeIn,
    ));

    *current_track = CurrentTrack::Menu;
}

fn setup_game_sound(
    mut commands: Commands,
    soundtrack_player: Res<SoundtrackPlayer>,
    mut current_track: ResMut<CurrentTrack>,
) {
    commands.spawn((
        AudioBundle {
            source: soundtrack_player.game_track.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::ZERO,
                ..Default::default()
            },
            ..Default::default()
        },
        FadeIn,
    ));

    *current_track = CurrentTrack::Game;
}

const FADE_TIME: f32 = 2.0;

fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
) {
    for (audio, entity) in &mut audio_sink {
        audio.set_volume(audio.volume() + time.delta_seconds() / FADE_TIME);
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (audio, entity) in &mut audio_sink {
        audio.set_volume(audio.volume() - time.delta_seconds() / FADE_TIME);
        if audio.volume() <= 0.0 {
            audio.set_volume(0.0);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn fade_out_menu(
    mut commands: Commands,
    current_track: ResMut<CurrentTrack>,
    menu_audio: Query<Entity, With<FadeIn>>,
) {
    if let CurrentTrack::Menu = *current_track {
        for entity in &menu_audio {
            commands.entity(entity).insert(FadeOut);
        }
    }
}

fn fade_out_game(
    mut commands: Commands,
    current_track: ResMut<CurrentTrack>,
    game_audio: Query<Entity, With<FadeIn>>,
) {
    if let CurrentTrack::Game = *current_track {
        for entity in &game_audio {
            commands.entity(entity).insert(FadeOut);
        }
    }
}

fn setup_sound(asset_server: Res<AssetServer>, mut commands: Commands) {
    let menu_track = asset_server.load("sounds/menu.ogg");
    let game_track = asset_server.load("sounds/game.ogg");

    commands.insert_resource(SoundtrackPlayer::new(menu_track, game_track));
    info!("SoundtrackPlayer initialized.");
}
