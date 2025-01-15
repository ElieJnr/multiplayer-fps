use crate::graphics::{resources::PlayerCountState, states::GameState};
use bevy::prelude::*;

use super::menu::despawn_menu;

#[derive(Component)]
struct OnWaittingScreen;

#[derive(Component)]
struct OnTransitionScreen;

#[derive(Component)]
struct FadeTimer(Timer);

pub struct WaittingRoomPlugin;

impl Plugin for WaittingRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Waitting), spawn_waitting_page)
            .add_systems(
                OnExit(GameState::Waitting),
                despawn_menu::<OnWaittingScreen>,
            )
            .add_systems(Update, (check_player_count, handle_transition_effect))
            .add_systems(OnEnter(GameState::Transition), spawn_transition_effect)
            .add_systems(
                OnExit(GameState::Transition),
                despawn_menu::<OnTransitionScreen>,
            );
    }
}

fn spawn_waitting_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnWaittingScreen,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Waiting for Players...".to_string(),
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    font: font.clone(),
                },
            ));
        });
}

fn check_player_count(
    player_count_state: Res<PlayerCountState>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if player_count_state.has_enough_players {
        game_state.set(GameState::Transition);
    }
}

fn spawn_transition_effect(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK),
                ..Default::default()
            },
            OnTransitionScreen,
            FadeTimer(Timer::from_seconds(3.0, TimerMode::Once)), // Fade-out duration
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Starting Game...".to_string(),
                TextStyle {
                    font_size: 70.0,
                    color: Color::WHITE,
                    font: font.clone(),
                },
            ));
        });
}

fn handle_transition_effect(
    time: Res<Time>,
    mut query: Query<(&mut FadeTimer, &mut Style, Entity), With<OnTransitionScreen>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (mut fade_timer, mut style, entity) in query.iter_mut() {
        fade_timer.0.tick(time.delta());

        let progress = fade_timer.0.elapsed_secs() / fade_timer.0.duration().as_secs_f32();
        style.aspect_ratio = Some(1.0 - progress); // Gradually decrease opacity for fade-out

        if fade_timer.0.finished() {
            // Transition complete, move to the game state
            commands.entity(entity).despawn_recursive();
            game_state.set(GameState::Game);
        }
    }
}
