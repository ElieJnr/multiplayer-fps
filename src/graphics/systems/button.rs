use bevy::prelude::*;

use crate::graphics::resources::PlayerCountState;

use super::menu::NORMAL_BUTTON_COLOR;

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    Play(bool),
    Options,
    Quit,
}

impl MenuButtonAction {
    pub fn from_label(label: &str) -> Self {
        match label {
            "Play" => MenuButtonAction::Play(false),
            "Options" => MenuButtonAction::Options,
            "Quit" => MenuButtonAction::Quit,
            _ => panic!("Invalid button label"),
        }
    }
}

pub fn spawn_menu_button(parent: &mut ChildBuilder, label: &str, assets_server: &AssetServer) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            },
            MenuButtonAction::from_label(label),
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: assets_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::BLACK,
                },
            ));
        });
}

pub fn update_play_button(
    player_count: Res<PlayerCountState>,
    mut query: Query<(&mut MenuButtonAction, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    let has_enough = player_count.has_enough_players;
    for (mut action, children) in query.iter_mut() {
        if let MenuButtonAction::Play(_) = *action {
            *action = MenuButtonAction::Play(has_enough);

            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    let label = if has_enough {
                        "Play"
                    } else {
                        "Waiting..."
                    };
                    text.sections[0].value = label.to_string();
                }
            }
        }
    }
}
