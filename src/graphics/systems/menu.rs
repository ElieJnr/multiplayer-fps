use crate::common::protocol::*;
use crate::graphics::network::NetworkConfig;
use crate::graphics::states::{GameState, MenuState};
use bevy::prelude::*;

use super::button::{spawn_menu_button, MenuButtonAction};

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5); 

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct OnOptionScreen;

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(OnExit(GameState::Menu), despawn_menu::<OnMenuScreen>)
        .add_systems(OnEnter(MenuState::Options), option_menu_setup)
        .add_systems(Update, button_interaction_system)
        .add_systems(Update, menu_action);
}


fn main_menu_setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    let icon = assets_server.load("game_icon.png");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    align_self: AlignSelf::Center,
                    justify_self: JustifySelf::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: icon,
                    ..Default::default()
                },
                style: Style {
                    width: Val::Px(500.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Add buttons
                    spawn_menu_button(parent, "Play", &assets_server);
                    spawn_menu_button(parent, "Options", &assets_server);
                    spawn_menu_button(parent, "Quit", &assets_server);
                });
        });
}


fn option_menu_setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Working in progress... ",
            TextStyle {
                font: assets_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        }),
        OnOptionScreen,
    ));
}

// Button interaction system to handle hover and press animations
fn button_interaction_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut query {
        *color = match *interaction {
            Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
            Interaction::Pressed => PRESSED_BUTTON_COLOR.into(),
            Interaction::None => NORMAL_BUTTON_COLOR.into(),
        };
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    network_config: Res<NetworkConfig>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    send_disconnect_message(&network_config.client_socket, &network_config.player_name, "You press quit");
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Options => menu_state.set(MenuState::Options),
            }
        }
    }
}

// Generic system that takes a Component as parameter, and will despawn all entities with that component
fn despawn_menu<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}