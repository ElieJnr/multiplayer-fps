use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::Timer,
};

// Composant marqueur pour identifier le texte des FPS
#[derive(Component)]
pub struct FpsText;

// Resource pour le timer de mise à jour des FPS
#[derive(Resource)]
pub struct FpsUpdateTimer {
    timer: Timer,
}

impl Default for FpsUpdateTimer {
    fn default() -> Self {
        Self {
            // Met à jour toutes les 0.5 secondes
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

/// Système pour configurer l'UI des FPS
pub fn setup_fps_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Initialiser le timer
    commands.insert_resource(FpsUpdateTimer::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "FPS: Calculating...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                FpsText,
            ));
        });
}

/// Système pour mettre à jour l'UI des FPS
pub fn update_fps_ui(
    time: Res<Time>,
    mut timer: ResMut<FpsUpdateTimer>,
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    // Mise à jour du timer
    timer.timer.tick(time.delta());

    // Ne met à jour que lorsque le timer a fini
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
