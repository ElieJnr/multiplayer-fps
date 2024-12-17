use bevy::prelude::*;
use bevy::ui::FocusPolicy;


// Button colors
const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_menu)
        .add_systems(Update, button_interaction_system)
        .run();
}

// ====================
// Menu UI Setup
// ====================

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background image
    commands.spawn(SpriteBundle {
        texture: asset_server.load("menu.png"),
        ..default()
    });

    // Spawn the title text
    commands.spawn(TextBundle::from_section(
        "Maze Wars FPS",
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 100.0,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(10.0)),
        position_type: PositionType::Absolute,
        justify_self: JustifySelf::Center,
        ..default()
    }));

    // Button container
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(400.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Add buttons
            spawn_menu_button(parent, "Play", &asset_server);
            spawn_menu_button(parent, "Options", &asset_server);
            spawn_menu_button(parent, "Quit", &asset_server);
        });
}

// Helper function to spawn a single menu button
fn spawn_menu_button(parent: &mut ChildBuilder, label: &str, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
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
        })
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


