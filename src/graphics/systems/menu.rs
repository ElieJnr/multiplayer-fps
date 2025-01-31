use crate::common::protocol::*;
use crate::graphics::resources::PlayerCountState;
use crate::graphics::states::{GameState, MenuState};
use bevy::prelude::*;
use bevy_tweening::{lens::*, Animator, EaseFunction, Tween};
use std::time::Duration;

use super::button::{spawn_menu_button, MenuButtonAction};

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const RED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.1, 0.1);

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
pub struct OnOptionScreen;

#[derive(Component)]
struct FadeTimer(Timer);

#[derive(Component)]
pub struct CloseHelpButton;

#[derive(Event)]
pub struct CloseHelpModalEvent;

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
        .add_event::<CloseHelpModalEvent>()
        .add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(OnExit(GameState::Menu), despawn_menu::<OnMenuScreen>)
        .add_systems(OnEnter(MenuState::Options), option_menu_setup)
        .add_systems(
            Update,
            (
                fade_in_system,
                button_interaction_system,
                menu_action,
                close_help_modal,
                handle_close_help_modal_event,
            ),
        );
}

fn main_menu_setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    let icon = assets_server.load("textures/game_icon.png");
    let blood_texture_handle = assets_server.load("textures/blood_splatter.png");
    commands.spawn((
        SpriteBundle {
            texture: blood_texture_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        OnMenuScreen,
    ));
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
                background_color: Color::NONE.into(),
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
                    spawn_menu_button(parent, "Play", &assets_server);
                    spawn_menu_button(parent, "Helps", &assets_server);
                    spawn_menu_button(parent, "Quit", &assets_server);
                });
        });
}

fn option_menu_setup(mut commands: Commands, _assets_server: Res<AssetServer>) {
    let instruction_asset = _assets_server.load("textures/instruction.png");
    commands
        .spawn((
            NodeBundle {
                background_color: Color::NONE.into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::all(Val::Auto),
                    width: Val::Percent(70.0),
                    height: Val::Percent(70.0),
                    flex_direction: FlexDirection::RowReverse,
                    ..Default::default()
                },
                z_index: ZIndex::Global(10),
                ..Default::default()
            },
            OnOptionScreen,
        ))
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: instruction_asset,
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),

                    ..Default::default()
                },
                ..Default::default()
            });
            parent
                .spawn(ButtonBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        width: Val::Px(50.0),
                        height: Val::Px(40.0),
                        position_type: PositionType::Relative,
                        right: Val::Percent(0.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        justify_self: JustifySelf::End,
                        align_self: AlignSelf::Start,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CloseHelpButton)
                .with_children(|button| {
                    button.spawn(TextBundle {
                        background_color: Color::NONE.into(),

                        text: Text::from_section(
                            "X",
                            TextStyle {
                                font: _assets_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

// System to handle button click and close the modal
pub fn close_help_modal(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, Entity),
        (Changed<Interaction>, With<CloseHelpButton>),
    >,
    mut close_event_writer: EventWriter<CloseHelpModalEvent>,
) {
    for (interaction, entity) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            close_event_writer.send(CloseHelpModalEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

// System to handle the close event
pub fn handle_close_help_modal_event(
    mut commands: Commands,
    mut close_event_reader: EventReader<CloseHelpModalEvent>,
    query: Query<Entity, With<OnOptionScreen>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for _event in close_event_reader.read() {
        for entity in query.iter() {
            menu_state.set(MenuState::Disabled);
            commands.entity(entity).despawn_recursive();
        }
    }
}

// Modified button interaction system to handle disabled state
fn button_interaction_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &mut Transform),
        (Changed<Interaction>, With<Button>, Without<CloseHelpButton>),
    >,
) {
    for (interaction, mut color, mut transform) in &mut query {
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
                MenuButtonAction::Play => {
                    game_state.set(GameState::Waitting);
                    send_ready_msg(&network_config, &network_config.player_name);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Options => menu_state.set(MenuState::Options),
            }
        }
    }
}

// Generic system that takes a Component as parameter, and will despawn all entities with that component
pub fn despawn_menu<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
