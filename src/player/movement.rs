use crate::client::player::Players;
use crate::common::protocol::{
    serialize_message, GameMessage, MessageContent, MessageType, NetworkConfig,
};
use crate::common::sync::NetworkMessages;
use crate::maze::barre_etat::GameStatus;
use crate::maze::models::*;
use crate::maze::models::{Collider, ColliderHouse, MazeState, ObstaclePositions};
use crate::player::model::*;
use crate::player::player::handle_keyboard_animation;
use crate::utils::logger::display_info;
use bevy::ecs::entity::Entity;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::prelude::{
    AnimationPlayer, Camera3d, Commands, EventReader, KeyCode, Local, Query, Res, ResMut,
    Transform, With, Without,
};

use bevy::scene::SceneBundle;
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::{CursorGrabMode, Window};

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut movement: ResMut<PlayerMovement>,
    _obstacle_positions: Res<ObstaclePositions>,
    network: Option<Res<NetworkConfig>>,
    mut sequence_number: Local<u32>,
    mut query: Query<&mut Transform, With<PlayersComponent>>,
    collider_query: Query<&Transform, (With<Collider>, Without<PlayersComponent>)>,
    house_collider_query: Query<&Transform, (With<ColliderHouse>, Without<PlayersComponent>)>,
    maze_state: Res<MazeState>,
    players: ResMut<Players>,
    game_status: ResMut<GameStatus>,
    animations: ResMut<PlayerAnimations>,
    animation_players: Query<&mut AnimationPlayer>,
    animation_query: Query<(Entity, &mut AnimationState)>,
) {
    if !maze_state.is_ready {
        return;
    }

    let mut mouse_delta = Vec2::ZERO;
    for event in motion_evr.read() {
        mouse_delta += event.delta;
    }

    simulation_tir(&keyboard_input, &network, &mut query, players, game_status);

    let input = PlayerInput {
        arrow_up: keyboard_input.pressed(KeyCode::KeyW),
        arrow_up_release: keyboard_input.just_released(KeyCode::KeyW),
        arrow_down: keyboard_input.pressed(KeyCode::KeyS),
        key_space: keyboard_input.pressed(KeyCode::Space),
        key_space_release: keyboard_input.just_released(KeyCode::Space),
        arrow_left: keyboard_input.pressed(KeyCode::KeyA),
        arrow_right: keyboard_input.pressed(KeyCode::KeyD),
        mouse_delta,
        ready: false,
    };
    if input.key_space || input.key_space_release {
        handle_keyboard_animation( animation_query, animations, animation_players);
    }

    if input.arrow_up
        || input.arrow_down
        || input.arrow_left
        || input.arrow_right
        || mouse_delta != Vec2::ZERO
    {
        *sequence_number += 1;
        let delta_time = time.delta_seconds();
        if let Ok(mut transform) = query.get_single_mut() {
            let new_transform = transform.clone();
            apply_input(&mut transform, &input, &movement, delta_time);

            if check_collisions(&transform, &collider_query, &house_collider_query) {
                *transform = new_transform;
            } else {
                movement.position = transform.translation;
            }
            movement.rotation.y = transform.rotation.y;
            let input_sequence = InputSequence {
                sequence_number: *sequence_number,
                timestamp: time.elapsed_seconds_f64(),
                input: input.clone(),
            };
            movement.input_buffer.push_back(input_sequence);
            if let Some(network) = network.as_ref() {
                let message = GameMessage {
                    message_type: MessageType::GameUpdate,
                    sender: network.player_name.clone(),
                    content: MessageContent::GameUpdate {
                        position: (transform.translation.x, transform.translation.z),
                        rotation: Vec2::new(
                            transform.rotation.to_euler(EulerRot::XYZ).0,
                            transform.rotation.to_euler(EulerRot::XYZ).1,
                        ),
                        sequence_number: *sequence_number,
                        timestamp: time.elapsed_seconds_f64(),
                        mouse_delta: input.mouse_delta,
                    },
                };
                if let Some(msg_bytes) = serialize_message(&message) {
                    let _ = network.client_socket.send(&msg_bytes);
                }
            }
        }
    }
}

fn simulation_tir(
    keyboard_input: &Res<'_, ButtonInput<KeyCode>>,
    network: &Option<Res<'_, NetworkConfig>>,
    query: &mut Query<'_, '_, &mut Transform, With<PlayersComponent>>,
    mut players: ResMut<'_, Players>,
    mut game_status: ResMut<GameStatus>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyT) {
        display_info("KeyT pressed, entering simulation_tir function");

        if players.0.is_empty() {
            display_info("No players found in the HashMap");
        } else {
            display_info(&format!("Number of players: {}", players.0.len()));
        }

        for (player_name, player) in players.0.iter_mut() {
            display_info(&format!("Processing player: {}", player_name));
            display_info(&format!("Player health before: {}", player.health));

            if player.health > 0 {
                player.health -= 1;
                game_status.player_health -= 0.2;
                display_info(&format!("Player health after: {}", player.health));
                display_info(&format!(
                    "Game status health: {}",
                    game_status.player_health
                ));

                if player.health == 0 {
                    // Remove the player
                    query.iter_mut().for_each(|mut transform| {
                        transform.translation = Vec3::new(0.0, -100.0, 0.0);
                    });

                    // Send game over message
                    if let Some(network) = network {
                        let game_over_msg = GameMessage {
                            message_type: MessageType::Disconnect,
                            sender: "server".to_string(),
                            content: MessageContent::ServerInfo {
                                server_status: format!(
                                    "Player {} has died. Game Over!",
                                    player_name
                                ),
                            },
                        };

                        if let Some(msg_bytes) = serialize_message(&game_over_msg) {
                            network.client_socket.send(&msg_bytes).unwrap();
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_input(
    transform: &mut Transform,
    input: &PlayerInput,
    movement: &PlayerMovement,
    delta_time: f32,
) {
    let forward = transform.forward();
    if input.arrow_up {
        transform.translation -= forward * movement.speed * delta_time;
    }
    if input.arrow_down {
        transform.translation += forward * movement.speed * delta_time;
    }
    if input.arrow_left {
        transform.translation += transform.right() * movement.speed * delta_time;
    }
    if input.arrow_right {
        transform.translation -= transform.right() * movement.speed * delta_time;
    }
    if input.mouse_delta.length_squared() > 0.0 {
        transform.rotate_y(-input.mouse_delta.x * movement.mouse_sensitivity);
    }
    transform.translation.y = movement.ground_level;
}

// permet de changer la vue de la camera
pub fn camera_view_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayersComponent>)>,
    mut camera_state: ResMut<CameraState>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        camera_state.is_top_view = !camera_state.is_top_view;
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            if camera_state.is_top_view {
                // Vue de haut
                camera_transform.translation = Vec3::new(0.0, 50.0, 0.0);
                camera_transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            } else {
                // Retour à la vue FPS initiale
                // println!("here");
                camera_transform.translation = Vec3::new(0.0, 0.5, -0.25);
                camera_transform.rotation = Transform::from_xyz(0.0, 0.5, -0.25)
                    .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y)
                    .rotation;
            }
        }
    }
}

// permet de rendre visible et invisible la souris avec la touche space
pub fn toggle_cursor_lock(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(mut window) = windows.get_single_mut() {
            if window.cursor.grab_mode == CursorGrabMode::Locked {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            } else {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}

pub fn check_collisions(
    player_transform: &Transform,
    collider_query: &Query<&Transform, (With<Collider>, Without<PlayersComponent>)>,
    house_collider_query: &Query<&Transform, (With<ColliderHouse>, Without<PlayersComponent>)>,
) -> bool {
    for collider_transform in collider_query.iter() {
        if collide(
            player_transform.translation,
            Vec3::new(0.6, 1.0, 0.6),
            collider_transform.translation,
            Vec3::new(1.0, 1.0, 1.0),
        )
        .is_some()
        {
            return true;
        }
    }
    for house_collider_transform in house_collider_query.iter() {
        if collide(
            player_transform.translation,
            Vec3::new(0.6, 1.0, 0.6),
            house_collider_transform.translation,
            Vec3::new(3.0, 2.5, 3.0),
        )
        .is_some()
        {
            return true;
        }
    }
    false
}

fn collide(pos1: Vec3, size1: Vec3, pos2: Vec3, size2: Vec3) -> Option<()> {
    let collision_x = (pos1.x - pos2.x).abs() < (size1.x + size2.x) / 2.0;
    let collision_y = (pos1.y - pos2.y).abs() < (size1.y + size2.y) / 2.0;
    let collision_z = (pos1.z - pos2.z).abs() < (size1.z + size2.z) / 2.0;
    if collision_x && collision_y && collision_z {
        Some(())
    } else {
        None
    }
}

pub fn manage_remote_players(
    mut commands: Commands,
    enemy_animations: Res<PreloadedEnemyAnimations>,
    enemy_graph: Res<EnemyAnimations>,
    mut remote_players: ResMut<RemotePlayers>,
    network: Res<NetworkConfig>,
    mut messages: ResMut<NetworkMessages>,
    mut query: Query<&mut Transform>,
) {
    while let Some(message) = messages.0.pop_front() {
        if let MessageContent::GameUpdate {
            position: (x, z),
            rotation,
            mouse_delta,
            ..
        } = &message.content
        {
            let player_name = &message.sender;
            if player_name == &network.player_name {
                continue;
            }
            if let Some(&entity) = remote_players.0.get(player_name) {
                if let Ok(mut transform) = query.get_mut(entity) {
                    transform.translation.x = *x;
                    transform.translation.z = *z;
                    transform.translation.y = 0.0;
                    transform.rotate_y(-mouse_delta.x * 0.003);
                }
            } else {
                let remote_player = commands
                    .spawn((
                        SceneBundle {
                            scene: enemy_animations.model.clone(),
                            transform: Transform {
                                translation: Vec3::new(*x, 0.0, *z),
                                rotation: Quat::from_euler(
                                    EulerRot::XYZ,
                                    0.0,
                                    rotation.y + std::f32::consts::PI,
                                    0.0,
                                ),
                                scale: Vec3::splat(0.5),
                                ..default()
                            },
                            ..default()
                        },
                        RemotePlayer {
                            name: player_name.clone(),
                        },
                        AnimationPlayer::default(),
                        enemy_graph.graph.clone(),
                        AnimationState::default(),
                    ))
                    .id();
                remote_players.0.insert(player_name.clone(), remote_player);
            }
        }
    }
}
