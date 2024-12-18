use bevy::{
    app::{App, Startup},
    prelude::{AppExtStates, Camera2dBundle, Commands, Component, Resource, States},
    DefaultPlugins,
};

// Enum that will b used as a global state for the game

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

// Elements of the Options that can be set through the menu. It will be a ressource in the app
#[derive(Debug, Resource, Component, PartialEq, Eq, Clone, Copy, Default)]
enum Map {
    #[default]
    Map00,
    // Map01,
    // Map02,
}

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Map::Map00)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu::menu_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

mod menu {
    use super::GameState;
    use bevy::{prelude::*, ui::FocusPolicy};
    // // Button colors
    const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
    const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
    const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

    // This plugin will display a menu screen  before swtching to the game

    pub fn menu_plugin(app: &mut App) {
        // as this plugin is managing the menu screen, it wil focus on the state `GameState::Menu`
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), main_menu_setup)
            .add_systems(
                OnExit(GameState::Menu),
                (
                    despawn_screen::<OnMenuScreen>,
                    despawn_screen::<OnOptionScreen>,
                ),
            )
            .add_systems(OnEnter(MenuState::Options), option_menu_setup)
            .add_systems(Update, (button_interaction_system, menu_action));
    }

    // Tag component used to tag entities
    #[derive(Component)]
    struct OnMenuScreen;

    #[derive(Component)]
    struct OnOptionScreen;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Options,
        Quit,
    }

    #[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash, States)]
    enum MenuState {
        // Main,
        Options,
        #[default]
        Disabled,
    }

    fn main_menu_setup(mut commands: Commands, assets_server: Res<AssetServer>) {
        let icon = assets_server.load("game_icon.png");

        //Display the logo

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

    // // Helper function to spawn a single menu button
    fn spawn_menu_button(parent: &mut ChildBuilder, label: &str, asset_server: &Res<AssetServer>) {
        let action = match label {
            "Play" => MenuButtonAction::Play,
            "Options" => MenuButtonAction::Options,
            "Quit" => MenuButtonAction::Quit,
            _ => panic!("Unexpected button label: {}", label),
        };

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                },
                action, // Attach the corresponding `MenuButtonAction` variant
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 25.0,
                        color: Color::BLACK,
                    },
                ));
            });
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
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
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

    // Generic system that takes a Component as parameter, and will despawn all entities with that component
    fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
        for entity in &to_despawn {
            commands.entity(entity).despawn_recursive();
        }
    }
}
