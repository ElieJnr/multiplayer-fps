use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::{lens::TransformScaleLens, Animator, EaseFunction, Tween};

use super::menu::NORMAL_BUTTON_COLOR;

#[derive(Component, Debug)]
pub enum MenuButtonAction {
    Play,
    Options,
    Quit,
}

impl MenuButtonAction {
    pub fn from_label(label: &str) -> Self {
        match label {
            "Play" => MenuButtonAction::Play,
            "Helps" => MenuButtonAction::Options,
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
                    // border: UiRect {
                    //     left: Val::Px(5.0),
                    //     right: Val::Px(5.0),
                    //     top: Val::Px(5.0),
                    //     bottom: Val::Px(5.0),
                    // },
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
        })
        .insert(Animator::new(Tween::new(
            EaseFunction::BounceInOut,
            Duration::from_millis(300),
            TransformScaleLens {
                start: Vec3::new(1.0, 1.0, 1.0),
                end: Vec3::new(1.2, 1.2, 1.0),
            },
        )));
}
