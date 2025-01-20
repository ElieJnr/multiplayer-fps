use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    color::Color,
    prelude::{
        BuildChildren, Commands, Component, NodeBundle, Query, Res, ResMut, Resource, TextBundle,
        With,
    },
    text::{Text, TextStyle},
    time::Time,
    ui::{AlignItems, BackgroundColor, Display, JustifyContent, PositionType, Style, UiRect, Val},
    utils::default,
};

use crate::{
    common::constant::PlayerCount,
    graphics::show_fps::{update_fps_ui, FpsText, FpsUpdateTimer},
};

#[derive(Debug, Resource, Default)]
pub struct GameStatus {
    pub time: u32,
    pub player_restant: usize,
    pub player_health: f32, // entre 0.0 et 1.0 donc pour 5 tir on decrementera a chaque tir 0.02
}

impl GameStatus {
    pub fn new(player_count: &PlayerCount) -> Self {
        Self {
            time: 60,
            player_restant: player_count.get_min_players(),
            player_health: 1.0,
        }
    }
}

#[derive(Component)]
pub struct GameStatusUI;

#[derive(Component)]
pub struct HealthBarFrame;

#[derive(Component)]
pub struct HealthBarFill;

pub struct GameStatusPlugin;

impl Plugin for GameStatusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameStatus>()
            .init_resource::<FpsUpdateTimer>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (update_game_status, update_health_bar, update_fps_ui),
            );
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.5, 0.7)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            display: Display::Flex,
                            justify_content: JustifyContent::SpaceBetween, 
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    GameStatusUI,
                ))
                .with_children(|status| {
                    status.spawn((
                        TextBundle::from_section(
                            "Players: 5/5",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        GameStatusUI,
                    ));

                    status.spawn((
                        TextBundle::from_section(
                            "FPS: Calculating...",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        FpsText,
                    ));

                    status.spawn((
                        TextBundle::from_section(
                            "Temps: 60s",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        GameStatusUI,
                    ));
                });

            // Barre de santé
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(20.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect {
                                left: Val::Px(50.0),
                                right: Val::Px(20.0),
                                top: Val::Auto, 
                                bottom: Val::Auto,
                            },
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                        ..default()
                    },
                    HealthBarFrame,
                ))
                .with_children(|health| {
                    health.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgb(0.0, 1.0, 0.0)),
                            ..default()
                        },
                        HealthBarFill,
                    ));
                });
        });
}

fn update_game_status(
    mut status: ResMut<GameStatus>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<GameStatusUI>>,
) {
    if status.time > 0 {
        status.time -= time.delta_seconds() as u32;
    }

    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!(
            "Temps: {}s | Joueurs: {}/5",
            status.time, status.player_restant
        );
    }
}

fn update_health_bar(status: Res<GameStatus>, mut query: Query<&mut Style, With<HealthBarFill>>) {
    if let Ok(mut style) = query.get_single_mut() {
        style.width = Val::Percent(status.player_health * 100.0);
    }
}
