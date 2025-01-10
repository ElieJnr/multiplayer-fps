use bevy::prelude::*;

use crate::maze::models::MinimapCamera;

pub fn minimap_setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0), // Ensure it can view the minimap
            camera: Camera {
                order: 1, // Ensure this camera renders on top of the 3D scene
                ..Default::default()
            },
            ..Default::default()
        },
        MinimapCamera, // Marker component for identification
    ));
}
