use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Handle},
    audio::{AudioBundle, AudioSink, AudioSinkPlayback, AudioSource, PlaybackSettings},
    prelude::{
        Commands, Component, DespawnRecursiveExt, DetectChanges, Entity, Query, Res, Resource, With,
    },
    time::Time,
};

use super::states::GameState;

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
        app.add_systems(Startup, setup)
            .add_systems(Update, (fade_in, fade_out))
            .add_systems(Update, change_track);
    }
}

//  This resource will hold the track list for your soundtrack
#[derive(Resource)]
struct SoundtrackPlayer {
    track_list: Vec<Handle<AudioSource>>,
}

impl SoundtrackPlayer {
    fn new(track_list: Vec<Handle<AudioSource>>) -> Self {
        Self { track_list }
    }
}

// This component will be attached to an entity to fade the audio in
#[derive(Component)]
struct FadeIn;

// This component will be attached to an entity to fade the audio out
#[derive(Component)]
struct FadeOut;

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Instantiate the game state resources
    commands.insert_resource(GameState::default());

    // Create the track list
    let track_1 = asset_server.load::<AudioSource>("sounds/menu.ogg");
    let track_2 = asset_server.load::<AudioSource>("sounds/game.ogg");
    let track_list = vec![track_1, track_2];
    commands.insert_resource(SoundtrackPlayer::new(track_list));
}

// Every time the GameState resource changes, this system is run to trigger the song change.
fn change_track(
    mut commands: Commands,
    soundtrack_player: Res<SoundtrackPlayer>,
    soundtrack: Query<Entity, With<AudioSink>>,
    game_state: Res<GameState>,
) {
    if game_state.is_changed() {
        // Fade out all currently running tracks
        for track in soundtrack.iter() {
            commands.entity(track).insert(FadeOut);
        }

        // Spawn a new `AudioPlayer` with the appropriate soundtrack based on
        // the game state.
        //
        // Volume is set to start at zero and is then increased by the fade_in system.
        match game_state.as_ref() {
            GameState::Menu => {
                commands.spawn((
                    AudioBundle {
                        source: soundtrack_player.track_list.first().unwrap().clone(),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::ZERO,
                            ..Default::default()
                        },
                    },
                    FadeIn,
                ));
            }

            GameState::Waitting => {
                commands.spawn((
                    AudioBundle {
                        source: soundtrack_player.track_list.get(1).unwrap().clone(),
                        settings: PlaybackSettings {
                            mode: bevy::audio::PlaybackMode::Loop,
                            volume: bevy::audio::Volume::ZERO,
                            ..Default::default()
                        },
                    },
                    FadeIn,
                ));
            }
            GameState::Game => {}
        }
    }
}

// Fade effect duration
const FADE_TIME: f32 = 2.0;

// Fades in the audio of entities that has the FadeIn component. Removes the FadeIn component once
// full volume is reached.
fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() + time.delta_seconds() / FADE_TIME);
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

// Fades out the audio of entities that has the FadeOut component. Despawns the entities once audio
// volume reaches zero.
fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        audio.set_volume(audio.volume() - time.delta_seconds() / FADE_TIME);
        if audio.volume() <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
