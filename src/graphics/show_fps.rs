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

#[derive(Component, Resource)]
pub struct Degat;

#[derive(Resource, Clone, Default)]
pub struct DamageFlashTimer {
    timer: Timer,
}

#[derive(Resource)]
pub struct DamageFlashActive(pub bool);

#[derive(Component)]
pub struct GameOver;

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

    commands.insert_resource(DamageFlashTimer {
        timer: Timer::from_seconds(1., TimerMode::Once),
    });
    commands.insert_resource(DamageFlashActive(false));
}

pub fn simulate_damage_flash(
    mut degat_query: Query<&mut Visibility, With<Degat>>,
    mut timer: ResMut<DamageFlashTimer>,
    time: Res<Time>,
    mut active: ResMut<DamageFlashActive>,
) {
    if !active.0 {
        return;
    }

    timer.timer.tick(time.delta());

    for mut visibility in &mut degat_query {
        if timer.timer.just_finished() {
            *visibility = Visibility::Hidden;
            active.0 = false;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

pub fn trigger_damage_flash(
    timer: &mut ResMut<DamageFlashTimer>,
    active: &mut ResMut<DamageFlashActive>,
) {
    active.0 = true;
    timer.timer.reset();
}

pub fn spawn_game_over_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_over = asset_server.load("textures/game_over.png");

    commands
        .spawn(ImageBundle {
            image: game_over.into(),
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(GameOver);
}
