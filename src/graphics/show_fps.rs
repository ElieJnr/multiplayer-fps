use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::Timer,
};

#[derive(Component)]
pub struct FpsText;

#[derive(Resource)]
pub struct FpsUpdateTimer {
    timer: Timer,
}

impl Default for FpsUpdateTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub fn update_fps_ui(
    time: Res<Time>,
    mut timer: ResMut<FpsUpdateTimer>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        if let Some(fps) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            for mut text in query.iter_mut() {
                text.sections[0].value = format!("FPS: {:.2}", fps);
            }
        }
    }
}

#[derive(Component)]
pub struct Degat;

pub fn show_degat(mut commands: Commands, asset_server: Res<AssetServer>) {
    let degat = asset_server.load("textures/degat.png");
    commands
        .spawn(ImageBundle {
            image: degat.into(),
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(Degat);
}
