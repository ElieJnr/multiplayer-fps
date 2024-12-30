use crate::common::protocol::*;
use crate::graphics::resources::PlayerCountState;
use crate::graphics::states::{GameState, MenuState};
use bevy::prelude::*;
use bevy_tweening::{lens::*, Animator, EaseFunction, Tween};
use std::time::Duration;

use super::button::{spawn_menu_button, update_play_button, MenuButtonAction};

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const RED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.1, 0.1);

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct OnOptionScreen;

#[derive(Component)]
struct FadeTimer(Timer);

fn fade_in_system(time: Res<Time>, mut query: Query<(&mut BackgroundColor, &mut FadeTimer)>) {
    for (mut color, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        let alpha = (timer.0.elapsed_secs() / timer.0.duration().as_secs_f32()).clamp(0.0, 1.0);
        let _ = color.0.set(Box::new(alpha));
    }
}

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .init_resource::<PlayerCountState>()
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(OnExit(GameState::Menu), despawn_menu::<OnMenuScreen>)
        .add_systems(OnEnter(MenuState::Options), option_menu_setup)
        .add_systems(
            Update,
            (
                fade_in_system,
                button_interaction_system,
                menu_action,
                update_play_button,
            ),
        );
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
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.0).into(), 
                ..Default::default()
            },
            OnMenuScreen,
            FadeTimer(Timer::from_seconds(1.5, TimerMode::Once)),
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
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    Animator::new(Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(1),
                        UiBackgroundColorLens {
                            start: Color::NONE,
                            end: Color::WHITE,
                        },
                    )),
                ))
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

// Modified button interaction system to handle disabled state
fn button_interaction_system(
    mut query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &MenuButtonAction,
            &mut Transform,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, action, mut transform) in &mut query {
        if let MenuButtonAction::Play(has_enough_players) = action {
            if !has_enough_players {
                *color = RED_BUTTON_COLOR.into();
                continue;
            }
        }

        *color = match *interaction {
            Interaction::Hovered => {
                transform.scale = Vec3::new(1.2, 1.2, 1.0);
                HOVERED_BUTTON_COLOR.into()
            }
            Interaction::Pressed => {
                transform.scale = Vec3::new(0.9, 0.9, 1.0);
                PRESSED_BUTTON_COLOR.into()
            }
            Interaction::None => {
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
                NORMAL_BUTTON_COLOR.into()
            }
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
                    send_disconnect_message(
                        &network_config,
                        &network_config.player_name,
                        "You press quit",
                    );
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play(has_enough_players) => {
                    if *has_enough_players {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
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
