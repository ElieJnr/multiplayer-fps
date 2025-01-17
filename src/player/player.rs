use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use serde_json::Value;
use std::{collections::HashMap, time::Duration};

pub struct PlayerBuild;

// impl PlayerBuild {
//     pub fn build(app: App) {
//         app.add_systems(Startup, (setup_player, preload_player_assets))
//             .add_systems(Update, (setup_player_animation, handle_keyboard_animation))
//     }
// }

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, (setup, preload_player_assets))
//         .add_systems(Update, (setup_player_animation, handle_keyboard_animation))
//         .run();
// }

#[derive(Component)]
pub struct AnimationState {
    current_animation: String,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: "idle".to_string(),
        }
    }
}

#[derive(Resource)]
pub struct PreloadedPlayerAnimations {
    pub model: Handle<Scene>,
    pub animations: HashMap<String, Handle<AnimationClip>>,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub player_entity: Entity,
    pub animation_player_entity: Option<Entity>,
    pub animations: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct Player;

// pub enum Animation {
//     Idle,
//     Run,
//     BackwardRun,
//     Shoot,
//     Reload
// }

// pub struct AnimationStr{

// }

// impl Animation{
//     fn create_animation()->Self{

//     }
// }

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Lumière
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_xyz(1.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     directional_light: DirectionalLight {
    //         illuminance: 50000.,
    //         ..Default::default()
    //     },
    //     ..default()
    // });

    // // Caméra
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0.0, 2.4, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

    // Plan
    commands.spawn(PbrBundle {
        mesh: meshes.add(Rectangle::new(50.0, 100.0)),
        material: materials.add(Color::srgb(1., 1., 1.)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn handle_keyboard_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut AnimationState)>,
    animations: ResMut<PlayerAnimations>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    let (_entity, mut animation_state) = query.single_mut();

    let new_animation = if keyboard.pressed(KeyCode::KeyW) {
        "run"
    } else if keyboard.pressed(KeyCode::KeyS) {
        "backward_run"
    } else if keyboard.pressed(KeyCode::KeyR) {
        "reload"
    } else if keyboard.pressed(KeyCode::Space) {
        "shoot"
    } else if keyboard.just_released(KeyCode::KeyW) {
        "stoprun"
    } else if keyboard.just_released(KeyCode::KeyS) {
        "stopbackward_run"
    } else if keyboard.just_released(KeyCode::Space) {
        "stopshoot"
    } else {
        "idle"
    };

    if new_animation == "reload" {
        if new_animation != animation_state.current_animation {
            animation_state.current_animation = new_animation.to_string();

            if let Some(player_entity) = animations.animation_player_entity {
                if let Ok(mut player) = animation_players.get_mut(player_entity) {
                    if let Some(&animation_index) = animations.animations.get(new_animation) {
                        let mut transitions = AnimationTransitions::new();
                        transitions.play(
                            &mut player,
                            animation_index,
                            Duration::from_secs_f32(0.2),
                        );
                    }
                }
            }
        }
    } else if !new_animation.starts_with("stop") && new_animation != "reload" {
        if new_animation != animation_state.current_animation {
            animation_state.current_animation = new_animation.to_string();

            if let Some(player_entity) = animations.animation_player_entity {
                if let Ok(mut player) = animation_players.get_mut(player_entity) {
                    if let Some(&animation_index) = animations.animations.get(new_animation) {
                        let mut transitions = AnimationTransitions::new();
                        transitions
                            .play(&mut player, animation_index, Duration::from_secs_f32(0.2))
                            .repeat();
                    }
                }
            }
        }
    } else if new_animation.starts_with("stop") {
        if let Some(player_entity) = animations.animation_player_entity {
            if let Ok(mut player) = animation_players.get_mut(player_entity) {
                if let Some(&run_animation) = animations
                    .animations
                    .get(new_animation.strip_prefix("stop").unwrap_or(""))
                {
                    player.stop(run_animation);
                }
            }
        }
    }
}

pub fn setup_player_animation(
    mut commands: Commands,
    mut animations: ResMut<PlayerAnimations>,
    mut animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut animation_players {
        info!("Setting up animation for player entity: {:?}", entity);
        let transitions = AnimationTransitions::new();
        if let Some(&idle_animation) = animations.animations.get("idle") {
            transitions
                .clone()
                .play(&mut player, idle_animation, Duration::ZERO)
                .repeat();
        }

        commands
            .entity(entity)
            .insert(animations.graph.clone())
            .insert(transitions.clone());
        animations.animation_player_entity = Some(entity);
    }
}

pub fn preload_player_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let default_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
    let path = format!("{}/assets/", default_path);
    let root = format!("{}player.glb", path);

    info!("Loading model from: {}", root);

    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(root.clone()));

    let mut animations = HashMap::new();
    let animation_names = ["idle", "run", "backward_run", "shoot", "reload"];

    for (i, &name) in animation_names.iter().enumerate() {
        info!("Loading animation: {} at index {}", name, i);
        animations.insert(
            name.to_string(),
            asset_server.load(GltfAssetLabel::Animation(i).from_asset(root.clone())),
        );
    }

    let mut graph = AnimationGraph::new();
    let mut animation_indices = HashMap::new();

    for (name, clip) in animations.iter() {
        info!("Adding animation {} to graph", name);
        let node_index = graph.add_clip(clip.clone(), 1.0, graph.root);
        animation_indices.insert(name.clone(), node_index);
    }

    let graph_handle = animation_graphs.add(graph);
    let map_data = include_str!("../maze/maze.json");
    let map: Value = serde_json::from_str(map_data).unwrap();
    let maze = map["maze-1"].as_array().unwrap();
    let height = maze.len() as f32;
    let width = maze[0].as_array().unwrap().len() as f32;
    info!("Spawning player entity");
    let player_entity = commands
        .spawn((
            SceneBundle {
                scene: model.clone(),
                transform: Transform {
                    translation: Vec3::new(width / 2.0, 0.0, height - 5.0),
                    scale: Vec3::splat(0.5), 
                    ..default()
                },
                ..default()
            },
            Player,
            AnimationPlayer::default(),
            graph_handle.clone(),
            AnimationState::default(),
        ))
        .id();

    commands.insert_resource(PreloadedPlayerAnimations { model, animations });

    commands.insert_resource(PlayerAnimations {
        player_entity,
        animation_player_entity: None,
        animations: animation_indices,
        graph: graph_handle,
    });

    info!("Player setup complete");
}
