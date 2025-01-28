use super::model::*;
use crate::maze::models::PlayersComponent;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

const TRANSITION_DURATION: f32 = 0.2;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub
 enum AnimationType {
    Repeating(String),
    OneShot(String),
    Stop(String),
}

pub fn handle_keyboard_animation(
    mut query: Query<(Entity, &mut AnimationState)>,
    mut animations: ResMut<PlayerAnimations>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    println!("-------------*handle_keyboard_animation-------------*");

    for (entity, mut animation_state) in query.iter_mut() {
        println!("Processing animation for entity: {:?}", entity);

        animations.player_entity = entity;

        let animation_type = AnimationType::OneShot("shoot".to_string());

        println!("animation_type:: {:?}", animation_type.clone());

        handle_animation(
            &animation_type,
            &mut animation_state,
            &animations,
            &mut animation_players,
        );
    }
}

// fn determine_animation_type(keyboard: &PlayerInput) -> Option<AnimationType> {
//     if keyboard.key_space {
//         Some(AnimationType::Repeating("shoot".to_string()));
//     } else if keyboard.key_space_release {
//         Some(AnimationType::Stop("shoot".to_string()));
//     }
//     Some(AnimationType::Repeating("static".to_string()))
// }

fn handle_animation(
    animation_type: &AnimationType,
    animation_state: &mut AnimationState,
    animations: &PlayerAnimations,
    animation_players: &mut Query<&mut AnimationPlayer>,
) {
    let player_entity = match animations.animation_player_entity {
        Some(entity) => entity,
        None => return,
    };

    let mut player = match animation_players.get_mut(player_entity) {
        Ok(player) => player,
        Err(_) => return,
    };

    match animation_type {
        AnimationType::Repeating(name) => {
            // if *name != animation_state.current_animation {
            play_animation(name, true, animation_state, animations, &mut player);
            // }
        }
        AnimationType::OneShot(name) => {
            // if *name != animation_state.current_animation {
            play_animation(name, false, animation_state, animations, &mut player);
            // }
        }
        AnimationType::Stop(name) => {
            if let Some(&animation_index) = animations.animations.get(name) {
                player.stop(animation_index);
            }
        }
    }
}

fn play_animation(
    name: &str,
    repeat: bool,
    animation_state: &mut AnimationState,
    animations: &PlayerAnimations,
    player: &mut AnimationPlayer,
) {
    if let Some(&animation_index) = animations.animations.get(name) {
        animation_state.current_animation = name.to_string();
        let mut transitions = AnimationTransitions::new();
        let transition = transitions.play(
            player,
            animation_index,
            Duration::from_secs_f32(TRANSITION_DURATION),
        );
        if repeat {
            transition.repeat();
        }
    }
}
pub fn setup_player_animation(
    mut commands: Commands,
    mut animations: ResMut<PlayerAnimations>,
    mut animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut animation_players {
        let transitions = AnimationTransitions::new();
        if let Some(&idle_animation) = animations.animations.get("static") {
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
    preload_assets(
        &mut commands,
        &asset_server,
        &mut animation_graphs,
        "player.glb",
        vec![
            "static",
            "arm",
            "ispect",
            "reload_fast",
            "reload_full",
            "run",
            "shoot",
            "walk",
        ],
        "PlayerAnimations",
    );

    preload_assets(
        &mut commands,
        &asset_server,
        &mut animation_graphs,
        "enemy.glb",
        vec!["idle", "run", "backward_run", "shoot", "reload"],
        "EnemyAnimations",
    );
}

fn preload_assets(
    commands: &mut Commands,
    asset_server: &AssetServer,
    animation_graphs: &mut Assets<AnimationGraph>,
    model_file: &str,
    animation_names: Vec<&str>,
    resource_name: &str,
) {
    let default_path = format!("{}", env!("CARGO_MANIFEST_DIR"));
    let path = format!("{}/assets/", default_path);
    let model_path = format!("{}{}", path, model_file);
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path.clone()));
    let mut animations = HashMap::new();
    for (i, &name) in animation_names.iter().enumerate() {
        animations.insert(
            name.to_string(),
            asset_server.load(GltfAssetLabel::Animation(i).from_asset(model_path.clone())),
        );
    }
    let mut graph = AnimationGraph::new();
    let mut animation_indices = HashMap::new();
    for (name, clip) in animations.iter() {
        let node_index = graph.add_clip(clip.clone(), 1.0, graph.root);
        animation_indices.insert(name.clone(), node_index);
    }
    let graph_handle = animation_graphs.add(graph);

    match resource_name {
        "PlayerAnimations" => {
            commands.insert_resource(PreloadedPlayerAnimations {
                model: model.clone(),
                animations: animations.clone(),
            });
            commands.insert_resource(PlayerAnimations {
                player_entity: Entity::from_raw(0),
                animation_player_entity: None,
                animations: animation_indices.clone(),
                graph: graph_handle.clone(),
            });
        }
        "EnemyAnimations" => {
            commands.insert_resource(PreloadedEnemyAnimations {
                model: model.clone(),
                animations: animations.clone(),
            });
            commands.insert_resource(EnemyAnimations {
                player_entity: Entity::from_raw(0),
                animation_player_entity: None,
                animations: animation_indices,
                graph: graph_handle,
            });
        }
        _ => {}
    }
}
pub fn create_players(
    commands: &mut Commands,
    player_animations: Res<PreloadedPlayerAnimations>,
    player_graph: Res<PlayerAnimations>,
    pos: Vec3,
) {
    let player_entity = commands
        .spawn((
            SceneBundle {
                scene: player_animations.model.clone(),
                transform: Transform {
                    translation: Vec3::new(pos[0], pos[1], pos[2]),
                    scale: Vec3::splat(0.25),
                    ..default()
                },
                ..default()
            },
            PlayerComponent,
            PlayersComponent,
            AnimationPlayer::default(),
            player_graph.graph.clone(),
            AnimationState::default(),
        ))
        .id();
    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, -0.25)
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
            ..default()
        },))
        .set_parent(player_entity);
    commands.insert_resource(PlayerAnimations {
        player_entity,
        animation_player_entity: None,
        animations: player_graph.animations.clone(),
        graph: player_graph.graph.clone(),
    });
}

pub fn create_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let bullet_mesh = meshes.add(Mesh::from(Cylinder {
        radius: 0.1,
        half_height: 0.5,
        ..Default::default()
    }));
    let bullet_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 1.0),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: bullet_mesh,
        material: bullet_material,
        transform: Transform::from_translation(position),
        ..Default::default()
    });
}
