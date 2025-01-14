use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use super::models::*;
use bevy::math::primitives::Cylinder;

#[derive(Resource)]
pub struct PlayerMovement {
    pub speed: f32,
    pub mouse_sensitivity: f32,
    pub ground_level: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            mouse_sensitivity: 0.003, 
            ground_level: 1.0,
        }
    }
}

pub fn create_player(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, width: f32, height: f32) -> Entity {
    let cylinder_mesh = meshes.add(Mesh::from(Cylinder { radius: 0.4, half_height: 0.5, ..default() }));
    let cylinder_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    let player = commands.spawn((
        PbrBundle {
            mesh: cylinder_mesh,
            material: cylinder_material,
            transform: Transform::from_xyz(width / 2.0, 1.0, height - 5.0),
            ..default()
        },
        Player,
        Collider,
    )).id();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.5, 0.0)
                .looking_at(Vec3::new(0.0, 0.5, 0.1), Vec3::Y), 
            ..default()
        },
    )).set_parent(player);

    player
}

pub fn player_movement(time: Res<Time>, keyboard_input: Res<ButtonInput<KeyCode>>, mut motion_evr: EventReader<MouseMotion>, mut query: Query<&mut Transform, With<Player>>, collider_query: Query<&Transform, (With<Collider>, Without<Player>)>, house_collider_query: Query<&Transform, (With<ColliderHouse>, Without<Player>)>, movement: Res<PlayerMovement>){
    let mut mouse_delta = Vec2::ZERO;
    for event in motion_evr.read() {
        mouse_delta += event.delta;
    }

    for mut transform in query.iter_mut() {
        let mut new_translation = transform.translation;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let forward = transform.forward();
            new_translation -= forward * movement.speed * time.delta_seconds();
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let forward = transform.forward();
            new_translation += forward * movement.speed * time.delta_seconds();
        }

        if !check_collisions(&Transform { translation: new_translation, ..*transform }, &collider_query, &house_collider_query) {
            transform.translation = new_translation;
        }

        if mouse_delta.length_squared() > 0.0 {
            transform.rotate_y(-mouse_delta.x * movement.mouse_sensitivity);
        }

        transform.translation.y = movement.ground_level;
    }
}

// permet de changer la vue de la camera
pub fn camera_view_toggle(keyboard_input: Res<ButtonInput<KeyCode>>, mut _player_query: Query<&mut Transform, With<Player>>, mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>, mut camera_state: ResMut<CameraState>) {
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
pub fn toggle_cursor_lock(keyboard_input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
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
    collider_query: &Query<&Transform, (With<Collider>, Without<Player>)>,
    house_collider_query: &Query<&Transform, (With<ColliderHouse>, Without<Player>)>,
) -> bool {
    for collider_transform in collider_query.iter() {
        if collide(
            player_transform.translation,
            Vec3::new(0.6, 1.0, 0.6), // player
            collider_transform.translation,
            Vec3::new(1.0, 1.0, 1.0), // obstacle wall
        ).is_some() {
            return true;
        }
    }

    for house_collider_transform in house_collider_query.iter() {
        if collide(
            player_transform.translation,
            Vec3::new(0.6, 1.0, 0.6), // player
            house_collider_transform.translation,
            Vec3::new(3.0, 2.5, 3.0), // obstacle house
        ).is_some() {
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