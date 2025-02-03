use crate::graphics::{resources::PlayerCountState, states::GameState};
use bevy::prelude::*;

use super::menu::despawn_menu;

#[derive(Component)]
struct OnWaittingScreen;

pub struct WaittingRoomPlugin;

impl Plugin for WaittingRoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Waitting), spawn_waitting_page)
            .add_systems(
                OnExit(GameState::Waitting),
                despawn_menu::<OnWaittingScreen>,
            )
            .add_systems(Update, check_player_count);
    }
}

fn spawn_waitting_page(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let img = asset_server.load("textures/waitting_img.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
            OnWaittingScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: img.into(),
                style: Style {
                    width: Val::Px(500.0),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            });
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
        game_state.set(GameState::Game);
    }
}
