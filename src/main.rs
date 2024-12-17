use bevy::prelude::*;
use bevy::ui::FocusPolicy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_menu)
        .add_systems(Update, button_hover_animation)
        .run();
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("menu.png"),

        ..default()
    });

    commands.spawn(TextBundle {
        text: Text::from_section(
            "Maze Wars FPS",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ),
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            position_type: PositionType::Absolute,
            justify_self: JustifySelf::Center,
            ..default()
        },
        ..default()
    });

    let button_style = Style {
        width: Val::Px(100.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        position_type: PositionType::Relative,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 25.0,
        color: Color::BLACK,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(400.0),
                height: Val::Px(400.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                align_self:AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            spawn_button(parent, &button_style, &button_text_style, "Play");
            spawn_button(parent, &button_style, &button_text_style, "Options");
            spawn_button(parent, &button_style, &button_text_style, "Quit");
        });
}

fn spawn_button(parent: &mut ChildBuilder, style: &Style, text_style: &TextStyle, label: &str) {
    parent
        .spawn(ButtonBundle {
            style: style.clone(),
            background_color: Color::srgb(0.8, 0.8, 0.8).into(),
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(label, text_style.clone()));
        });
}

fn button_hover_animation(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Style, Entity),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _style, _entity) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = Color::srgb(0.6, 0.6, 0.6).into();
            }
            Interaction::None | Interaction::Pressed => {
                *color = Color::srgb(0.8, 0.8, 0.8).into();
            }
        }
    }
}
