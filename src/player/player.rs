use super::{model::*, movement::get_player_input};
use crate::maze::models::{ColliderEnemy, PlayersComponent};
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use std::{collections::HashMap, time::Duration};

pub fn handle_keyboard_animation(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<(Entity, &mut AnimationState)>, animations: ResMut<PlayerAnimations>, mut animation_players: Query<&mut AnimationPlayer>) {
    if let Ok((_entity, mut animation_state)) = query.get_single_mut() {
        let input = get_player_input(keyboard, Vec2::ZERO);

        let new_animation = match (input.arrow_up || input.arrow_down, input.shoot, input.reload) {
            (true, _, _) => "walk",
            (_, true, _) => "shoot",
            (_, _, true) => "reload_fast",
            _ => "static",
        };

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
    }
}

pub fn setup_player_animation(mut commands: Commands, mut animations: ResMut<PlayerAnimations>, mut animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>) {
    for (entity, mut player) in &mut animation_players {
        // info!("Setting up animation for player entity: {:?}", entity);
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

pub fn preload_player_assets(mut commands: Commands, asset_server: Res<AssetServer>, mut animation_graphs: ResMut<Assets<AnimationGraph>>) {
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

fn preload_assets(commands: &mut Commands, asset_server: &AssetServer, animation_graphs: &mut Assets<AnimationGraph>, model_file: &str, animation_names: Vec<&str>, resource_name: &str) {
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

pub fn create_players(commands: &mut Commands, player_animations: Res<PreloadedPlayerAnimations>, player_graph: Res<PlayerAnimations>, pos: Vec3) {
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

pub fn shoot_bullet(
    commands: &mut Commands,
    bullet_resources: Res<BulletResources>,
    camera_transform: &Transform,
) {
    let bullet_direction = camera_transform.forward();
    let bullet_start_position = camera_transform.translation + bullet_direction * -0.2 + Vec3::new(0.0, 0.1, 0.0);

    commands.spawn((
        PbrBundle {
            mesh: bullet_resources.mesh.clone(),
            material: bullet_resources.material.clone(),
            transform: Transform {
                translation: bullet_start_position,
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                ..Default::default()
            },
            ..Default::default()
        },
        ColliderEnemy,
        Bullet {
            direction: *bullet_direction,
            speed: 2.0,
        },
    ));
}
