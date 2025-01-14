use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{
    BuildChildren, Camera3d, Camera3dBundle, Commands, Cylinder, Entity, EventReader, KeyCode,
    Mesh, Query, Res, ResMut, Resource, Transform, With, Without,
};
use bevy::time::Time;
use bevy::utils::default;
use bevy::window::{CursorGrabMode, Window};

use crate::common::protocol::{
    serialize_message, GameMessage, MessageContent, MessageType, NetworkConfig,
};
use crate::common::sync::NetworkMessages;

use super::models::*;

#[derive(Resource, Debug, Clone)]
pub struct PlayerMovement {
    pub speed: f32,
    pub mouse_sensitivity: f32,
    pub ground_level: f32,
    pub position: Vec3,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            mouse_sensitivity: 0.003,
            ground_level: 1.0,
            position: Vec3::ZERO,
        }
    }
}

impl PlayerMovement {
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
    }
}

pub fn create_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    width: f32,
    height: f32,
) -> Entity {
    let cylinder_mesh = meshes.add(Mesh::from(Cylinder {
        radius: 0.4,
        half_height: 0.5,
        ..default()
    }));
    let cylinder_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    let player = commands
        .spawn((
            PbrBundle {
                mesh: cylinder_mesh,
                material: cylinder_material,
                transform: Transform::from_xyz(width / 2.0, 1.0, height - 5.0),
                ..default()
            },
            Player,
        ))
        .id();

    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0)
                .looking_at(Vec3::new(0.0, 0.5, 0.1), Vec3::Y),
            ..default()
        },))
        .set_parent(player);

    player
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
    mut movement: ResMut<PlayerMovement>,
    network: Option<Res<NetworkConfig>>,
) {
    let mut mouse_delta = Vec2::ZERO;
    for event in motion_evr.read() {
        mouse_delta += event.delta;
    }

    for mut transform in query.iter_mut() {
        let mut moved = false;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let forward = transform.forward();
            transform.translation -= forward * movement.speed * time.delta_seconds();
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let forward = transform.forward();
            transform.translation += forward * movement.speed * time.delta_seconds();
            moved = true;
        }

        if mouse_delta.length_squared() > 0.0 {
            transform.rotate_y(-mouse_delta.x * movement.mouse_sensitivity);
            moved = true;
        }

        transform.translation.y = movement.ground_level;
        
        if moved {
            movement.set_position(transform.translation);

            if let Some(network) = network.as_ref() {
                let message = GameMessage {
                    message_type: MessageType::GameUpdate,
                    sender: network.player_name.clone(),
                    content: MessageContent::GameUpdate {
                        position: (movement.position.x, movement.position.z),
                        score: 0,
                    },
                };

                if let Some(msg_bytes) = serialize_message(&message) {
                    let _ = network.client_socket.send(&msg_bytes);
                }
            }
        }
    }
}

// permet de changer la vue de la camera
pub fn camera_view_toggle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut _player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut camera_state: ResMut<CameraState>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        camera_state.is_top_view = !camera_state.is_top_view;

        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            if camera_state.is_top_view {
                camera_transform.translation = Vec3::new(0.0, 50.0, 0.0);
                camera_transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            } else {
                camera_transform.translation = Vec3::new(0.0, 0.5, 0.0);
                camera_transform.rotation = Quat::IDENTITY;
                camera_transform.look_at(Vec3::new(0.0, 0.5, 0.1), Vec3::Y);
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

pub fn manage_remote_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut remote_players: ResMut<RemotePlayers>,
    network: Res<NetworkConfig>,
    mut messages: ResMut<NetworkMessages>,
) {
    while let Some(message) = messages.0.pop_front() {
        println!("player sim {:#?}", message);

        if let MessageContent::GameUpdate {
            position: (x, z), ..
        } = &message.content
        {
            let player_name = &message.sender;

            if player_name == &network.player_name {
                continue;
            }

            if let Some(&entity) = remote_players.0.get(player_name) {
                if let Some(mut entity_commands) = commands.get_entity(entity) {
                    entity_commands.insert(Transform::from_xyz(*x, 1.0, *z));
                }
            } else {
                let remote_player = commands
                    .spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(Cylinder {
                                radius: 0.4,
                                half_height: 0.5,
                                ..default()
                            })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::srgb(1.0, 0.0, 0.0),
                                ..default()
                            }),
                            transform: Transform::from_xyz(*x, 1.0, *z),
                            ..default()
                        },
                        RemotePlayer {
                            name: player_name.clone(),
                        },
                    ))
                    .id();

                remote_players.0.insert(player_name.clone(), remote_player);
            }
        }
    }
}
