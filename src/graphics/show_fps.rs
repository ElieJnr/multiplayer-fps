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

