use bevy::prelude::*;
use super::models::*;
use bevy::math::primitives::Cylinder;

#[derive(Resource)]
pub struct PlayerMovement {
    pub speed: f32,
    pub rotation_speed: f32,
    pub ground_level: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 5.0,
            rotation_speed: 2.0,
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

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    movement: Res<PlayerMovement>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            let forward = transform.forward();
            transform.translation -= forward * movement.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            let forward = transform.forward();
            transform.translation += forward * movement.speed * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.rotate_y(movement.rotation_speed * time.delta_seconds());
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.rotate_y(-movement.rotation_speed * time.delta_seconds());
        }
        transform.translation.y = movement.ground_level; 
    }
}

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
                // Top view
                camera_transform.translation = Vec3::new(0.0, 50.0, 0.0);
                camera_transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            } else {
                // FPS view
                camera_transform.translation = Vec3::new(0.0, 0.5, 0.0); 
                camera_transform.rotation = Quat::IDENTITY;
                camera_transform.look_at(Vec3::new(0.0, 0.5, 0.1), Vec3::Y); 
            }
        }
    }
}