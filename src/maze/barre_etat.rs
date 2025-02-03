use std::collections::HashMap;

use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    color::Color,
    prelude::{
        BuildChildren, ChildBuilder, Commands, Component, NodeBundle, OnEnter, Query, Res,
        Resource, State, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, BorderColor, BorderRadius, Display, FlexDirection,
        JustifyContent, PositionType, Style, UiImage, UiRect, Val,
    },
    utils::default,
};

use crate::{
    client::player::Player,
    graphics::{
        show_fps::{update_fps_ui, FpsText, FpsUpdateTimer},
        states::GameState,
    },
};

#[derive(Resource)]
pub struct PlayersResource(pub HashMap<String, Player>);

use super::models::MazeState;

#[derive(Debug, Resource, Default)]
pub struct GameStatus {
    pub player_health: f32,
}

impl GameStatus {
    pub fn new() -> Self {
        Self { player_health: 1. }
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
            .init_resource::<MazeState>()
            .add_systems(OnEnter(GameState::Game), setup_ui)
            .add_systems(Update, (update_health_bar, update_fps_ui));
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get() != &GameState::Game {
        return;
    }

    commands.insert_resource(GameStatus::new());

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                height: Val::Px(65.0),
                left: Val::Percent(0.0),
                right: Val::Percent(0.0),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            setup_status_ui(parent, &asset_server);
            setup_health_bar_ui(parent, &asset_server);
        });
}

fn setup_status_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
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
            spawn_fps_status(status, asset_server);
        });
}

fn spawn_fps_status(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        TextBundle::from_section("FPS: Calculating...", create_text_style(asset_server)),
        FpsText,
    ));
}

fn create_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    }
}

fn setup_health_bar_ui(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_health_icon(parent, asset_server);
            spawn_health_bar(parent);
        });
}

fn spawn_health_icon(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(45.0),
                height: Val::Px(45.0),
                margin: UiRect {
                    left: Val::Px(250.0),
                    ..default()
                },
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..default()
        },
        UiImage {
            texture: asset_server.load("textures/icône_vie.png"),
            color: Color::WHITE,
            flip_x: true,
            flip_y: false,
        },
    ));
}

fn spawn_health_bar(parent: &mut ChildBuilder) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(250.0),
                    height: Val::Px(25.0),
                    border: UiRect::all(Val::Px(2.0)),
                    margin: UiRect {
                        left: Val::Px(2.),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(20.0)),
                background_color: BackgroundColor(Color::srgb(0.96, 0.7, 0.45)),
                border_color: BorderColor(Color::srgb(1.0, 0.0, 0.3)),
                ..default()
            },
            HealthBarFrame,
        ))
        .with_children(|parent| {
            spawn_health_fill(parent);
        });
}

fn spawn_health_fill(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(20.0)),
            background_color: BackgroundColor(Color::srgb(1., 0.0, 0.0)),
            border_color: BorderColor(Color::srgb(0.0, 1.0, 1.0)),
            ..default()
        },
        HealthBarFill,
    ));
}

fn update_health_bar(
    game_status: Res<GameStatus>,
    mut query: Query<&mut Style, With<HealthBarFill>>,
) {
    if let Ok(mut style) = query.get_single_mut() {
        style.width = Val::Percent(game_status.player_health * 100.0);
    }
}
