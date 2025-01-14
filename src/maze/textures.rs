use super::models::*;
use bevy::math::primitives::{Cylinder, Sphere};
use bevy::prelude::*;
use std::f32::consts::PI;

// permet de créer un arbre procedural en utilisant les paramètres spécifiés
pub fn create_procedural_tree(commands: &mut Commands, _meshes: &mut ResMut<Assets<Mesh>>, _materials: &mut ResMut<Assets<StandardMaterial>>, params: &TreeParams, position: Vec3, branch_material: Handle<StandardMaterial>, leaf_material: Handle<StandardMaterial>, branch_mesh: Handle<Mesh>, leaf_mesh: Handle<Mesh>) {
    let tree = generate_tree(params);
    let mut entity_parent_indices: Vec<(Entity, Option<usize>)> = Vec::new();

    // Créer l'entité racine de l'arbre avec la position spécifiée
    let root = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..default()
            },
            ProceduralTree,
        ))
        .id();

    // Générer toutes les branches et feuilles
    for branch in &tree {
        let entity_id = if branch.2 {
            // Si c'est une feuille
            commands
                .spawn(PbrBundle {
                    mesh: leaf_mesh.clone(),
                    material: leaf_material.clone(),
                    transform: branch.0,
                    ..default()
                })
                .id()
        } else {
            // Si c'est une branche
            commands
                .spawn(PbrBundle {
                    mesh: branch_mesh.clone(),
                    material: branch_material.clone(),
                    transform: branch.0,
                    ..default()
                })
                .id()
        };
        entity_parent_indices.push((entity_id, branch.1));
    }

    // Établir les relations parent-enfant
    for (child_id, parent_idx) in &entity_parent_indices {
        if let Some(parent_idx) = parent_idx {
            let parent_id = entity_parent_indices[*parent_idx].0;
            commands.entity(parent_id).push_children(&[*child_id]);
        } else {
            commands.entity(root).push_children(&[*child_id]);
        }
    }
}

// permet de générer l'arbre
pub fn generate_tree(params: &TreeParams) -> Vec<Branch> {
    let base = Transform::default();
    let mut ret: Vec<Branch> = Vec::new();
    ret.push(Branch(base, None, false));
    generate_branches(params, 1, 0, &mut ret);
    ret
}

// permet de générer les branches
pub fn generate_branches(params: &TreeParams, level: u8, parent_idx: usize, all: &mut Vec<Branch>) {
    for i in 0..params.children {
        let angle_from_root_branch = params.angle_from_parent_branch;
        let child_gap_f32 = f32::from(i) / f32::from(params.children);
        let angle_around_root_branch = 2.0 * PI * child_gap_f32;
        let child_idx_f32 = f32::from(i) / f32::from(params.children - 1);

        let translation_along_root =
            (1.0 - child_idx_f32) * params.child_translation_factor + child_idx_f32;

        let mut child_transform = Transform::IDENTITY;
        child_transform.rotate_local_y(angle_around_root_branch);
        child_transform = child_transform.with_translation(
            child_transform.local_z()
                * (params.base_radius + params.child_scale * 0.5 * angle_from_root_branch.sin())
                + child_transform.local_y()
                    * ((translation_along_root - 0.5)
                        + params.child_scale * 0.5 * angle_from_root_branch.cos()),
        );
        child_transform = child_transform.with_scale(Vec3::splat(params.child_scale));
        child_transform.rotate_local_x(angle_from_root_branch);

        let child_idx = all.len();
        all.push(Branch(child_transform, Some(parent_idx), false));

        if level < params.levels {
            generate_branches(params, level + 1, child_idx, all);
        } else {
            generate_leaves(child_idx, all);
        }
    }
}

// permet de générer les feuilles
pub fn generate_leaves(parent_idx: usize, all: &mut Vec<Branch>) {
    let mut child_transform = Transform::IDENTITY;
    child_transform = child_transform.with_translation(*child_transform.local_y());
    all.push(Branch(child_transform, Some(parent_idx), true));
}

// permet de créer le sol en utilisant une taille de 0.1 et une texture de sol
pub fn create_surface(commands: &mut Commands, floor_mesh: Handle<Mesh>, floor_material: Handle<StandardMaterial>, width: f32, height: f32) {
    commands.spawn(PbrBundle {
        mesh: floor_mesh.clone(),
        material: floor_material.clone(),
        transform: Transform::from_xyz(width / 2.0, 0.0, height / 2.0),
        ..default()
    });
}

// permet de créer les murs en utilisant une taille de 1.0, une largeur de 2.0 et une hauteur de 1.0 tous en les positionnant correctement aux coordonnées i et j
pub fn create_walls(commands: &mut Commands, wall_mesh: Handle<Mesh>, wall_material: Handle<StandardMaterial>, i: usize, j: usize) {
    commands.spawn((
        PbrBundle {
            mesh: wall_mesh.clone(),
            material: wall_material.clone(),
            transform: Transform::from_xyz(j as f32, 1.0, i as f32),
            ..default()
        },
        Collider,
    ));
}

// permet de créer les lumières en les positionnant au centre de la scène
pub fn create_lights(commands: &mut Commands, width: f32, height: f32) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 90000000.0,
            range: 100.0,
            color: Color::srgb(1.0, 0.9, 0.8),
            ..default()
        },
        // transform: Transform::from_xyz(-15.0, 28.0, 5.0),
        transform: Transform::from_xyz(width / 2.0, 28.0, height / 2.0),
        ..default()
    });
}

// permet de creer le ciel en utilisant une sphere de rayon 500.0 et en lui appliquant une texture de ciel
pub fn create_sky(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, sky_texture: Handle<Image>) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: 500.0 })),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(sky_texture),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(-1.0)), // Invert the sphere
            ..default()
        },
        Sky,
    ));
}

// permet de faire tourner le ciel
pub fn rotate_sky(time: Res<Time>, mut query: Query<&mut Transform, With<Sky>>) {
    let rotation = Quat::from_rotation_y(time.elapsed_seconds() * 0.05);
    for mut transform in &mut query {
        transform.rotation = rotation;
    }
}

// permet de créer un arc en utilisant des piliers et un arc supérieur
pub fn create_arch(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    arch_mesh: Handle<Mesh>,
    sup_arch_mesh: Handle<Mesh>,
    arch_material: Handle<StandardMaterial>,
    arch: Entity,
    pillar_height: f32,
    arch_radius: f32,
    i: f32,
    j: f32,
) {
    let transform = if i == 29.0 && j == 22.0 {
        Transform::from_xyz(j - 0.26, 0.0, i + 0.5)
            .with_rotation(Quat::from_rotation_y(PI / 2.0))
    } else if i == 30.0 && j == 22.0 {
        Transform::from_xyz(j + 0.26, 0.0, i - 0.5)
            .with_rotation(Quat::from_rotation_y(PI / 2.0))
    } else {
        Transform::from_xyz(j - 0.5, 0.0, i + 0.28)
            .with_rotation(Quat::from_rotation_y(PI * 2.0))
    };

    commands.entity(arch).insert(transform);

    // Piliers
    for x in [-arch_radius, arch_radius] {
        commands.spawn((
            PbrBundle {
                mesh: arch_mesh.clone(),
                material: arch_material.clone(),
                transform: Transform::from_xyz(x, pillar_height / 2.0, 0.0),
                ..default()
            },
        )).set_parent(arch);
    }

    // Arc supérieur
    let segments = 16;
    for i in 0..segments {
        let angle = PI * (i as f32) / (segments - 1) as f32;
        let x = angle.cos() * arch_radius;
        let y = angle.sin() * arch_radius + pillar_height;

        commands
            .spawn((
                PbrBundle {
                    mesh: sup_arch_mesh.clone(),
                    material: arch_material.clone(),
                    transform: Transform::from_xyz(x, y, 0.0)
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            },
        ))
            .set_parent(arch);
    }
}
// permet de créer une maison en utilisant 4 facades de mur
pub fn create_house(commands: &mut Commands, meshes: Handle<Mesh>, house_materials: &HouseMaterials, position: Vec3) {
    let wall_size = Vec3::new(3.0, 2.5, 0.1);

    let (front_material, back_material, left_material, right_material) = if position.x == 7.0
        && position.z == 29.0
    {
        house_materials.house_2.clone()
    } else if position.x == 2.0 && position.z == 16.0 || position.x == 7.0 && position.z == 16.0 {
        house_materials.house_3.clone()
    } else if position.x == 4.0 && position.z == 34.0 {
        house_materials.house_4.clone()
    } else if position.x == 2.0 && position.z == 2.0
        || position.x == 17.0 && position.z == 8.0
        || position.x == 20.0 && position.z == 2.0
        || position.x == 10.0 && position.z == 8.0
    {
        house_materials.house_5.clone()
    } else if position.x == 4.0 && position.z == 12.0 {
        house_materials.house_6.clone()
    } else if position.x == 9.0 && position.z == 36.0 {
        house_materials.house_8.clone()
    } else if position.x == 12.0 && position.z == 36.0 {
        house_materials.house_7.clone()
    } else {
        house_materials.house_1.clone()
    };

    let house = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    wall_size.y / 8.6,
                    position.z,
                )),
                ..default()
            },
            ColliderHouse,
        ))
        .id();

    // Front face
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.clone(),
                material: front_material,
                transform: Transform::from_xyz(0.0, 1.0, 1.5),
                ..default()
            },
            ColliderHouse,
        ))
        .set_parent(house);

    // Back face
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.clone(),
                material: back_material,
                transform: Transform::from_xyz(0.0, 1.0, -1.5),
                ..default()
            },
            ColliderHouse,
        ))
        .set_parent(house);

    // Left face
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.clone(),
                material: left_material,
                transform: Transform::from_xyz(-1.5, 1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            ColliderHouse,
        ))
        .set_parent(house);

    // Right face
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.clone(),
                material: right_material,
                transform: Transform::from_xyz(1.5, 1.0, 0.0)
                    .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            ColliderHouse,
        ))
        .set_parent(house);
}

// permet de charger les textures
pub fn load_textures(asset_server: &Res<AssetServer>) -> Textures {
    Textures {
        wall_texture: asset_server.load("textures/wall_2.png"),
        arch_texture: asset_server.load("textures/wall.png"),
        floor_texture: asset_server.load("textures/floor_sand.png"),
        sky_texture: asset_server.load("textures/cloud_2.png"),
        house_textures: HouseTextures {
            house_1: (
                asset_server.load("textures/house_1_front.png"),
                asset_server.load("textures/house_1_back.png"),
                asset_server.load("textures/house_1_left.png"),
                asset_server.load("textures/house_1_right.png"),
            ),
            house_2: (
                asset_server.load("textures/house_2_back.png"),
                asset_server.load("textures/house_2.png"),
                asset_server.load("textures/house_2_left.png"),
                asset_server.load("textures/house_2_right.png"),
            ),
            house_3: (
                asset_server.load("textures/house_3_front.png"),
                asset_server.load("textures/house_3_back.png"),
                asset_server.load("textures/house_3_left.png"),
                asset_server.load("textures/house_3_right.png"),
            ),
            house_4: (
                asset_server.load("textures/house_4_back.png"),
                asset_server.load("textures/house_4.png"),
                asset_server.load("textures/house_4_left.png"),
                asset_server.load("textures/house_4_right.png"),
            ),
            house_5: (
                asset_server.load("textures/house_5_front_1.png"),
                asset_server.load("textures/house_5_back.png"),
                asset_server.load("textures/house_5_left.png"),
                asset_server.load("textures/house_5_right.png"),
            ),
            house_6: (
                asset_server.load("textures/house_5_back.png"),
                asset_server.load("textures/house_5_front.png"),
                asset_server.load("textures/house_5_left.png"),
                asset_server.load("textures/house_5_right.png"),
            ),
            house_7: (
                asset_server.load("textures/house_7.png"),
                asset_server.load("textures/house_7_front.png"),
                asset_server.load("textures/house_7.png"),
                asset_server.load("textures/house_7.png"),
            ),
            house_8: (
                asset_server.load("textures/house_7.png"),
                asset_server.load("textures/house_8_front.png"),
                asset_server.load("textures/house_7.png"),
                asset_server.load("textures/house_7.png"),
            ),
        },
    }
}

pub fn initialize_materials(meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, textures: &Textures, params: &TreeParams, width: f32, height: f32) -> (Handle<Mesh>, HouseMaterials, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<Mesh>, Handle<Mesh>, Handle<StandardMaterial>, Handle<Mesh>, Handle<Mesh>, Handle<StandardMaterial>, Handle<Mesh>, Handle<Mesh>, Handle<StandardMaterial>) {
    // house materials
    let wall_size = Vec3::new(3.0, 2.5, 0.1);
    let mesh_face = meshes.add(Mesh::from(Cuboid::new(
        wall_size.x,
        wall_size.y,
        wall_size.z,
    )));
    let mut house_materials = HouseMaterials::default();

    for i in 1..=8 {
        let house_textures = match i {
            1 => &textures.house_textures.house_1,
            2 => &textures.house_textures.house_2,
            3 => &textures.house_textures.house_3,
            4 => &textures.house_textures.house_4,
            5 => &textures.house_textures.house_5,
            6 => &textures.house_textures.house_6,
            7 => &textures.house_textures.house_7,
            8 => &textures.house_textures.house_8,
            _ => panic!("Unexpected house index: {}", i),
        };

        let materials_tuple = (
            materials.add(StandardMaterial {
                base_color_texture: Some(house_textures.0.clone()),
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color_texture: Some(house_textures.1.clone()),
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color_texture: Some(house_textures.2.clone()),
                ..default()
            }),
            materials.add(StandardMaterial {
                base_color_texture: Some(house_textures.3.clone()),
                ..default()
            }),
        );

        match i {
            1 => house_materials.house_1 = materials_tuple,
            2 => house_materials.house_2 = materials_tuple,
            3 => house_materials.house_3 = materials_tuple,
            4 => house_materials.house_4 = materials_tuple,
            5 => house_materials.house_5 = materials_tuple,
            6 => house_materials.house_6 = materials_tuple,
            7 => house_materials.house_7 = materials_tuple,
            8 => house_materials.house_8 = materials_tuple,
            _ => (),
        }
    }

    let branch_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    let leaf_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.8, 0.3),
        ..default()
    });

    let branch_mesh = meshes.add(Cylinder {
        radius: params.base_radius,
        half_height: 0.5,
    });

    let leaf_mesh = meshes.add(Sphere {
        radius: params.leaf_radius,
    });

    // wall materials
    let wall_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 2.0, 1.0)));
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(textures.wall_texture.clone()),
        ..default()
    });

    // surface materials
    let floor_mesh = meshes.add(Mesh::from(Cuboid::new(width, 0.1, height)));
    let floor_material = materials.add(StandardMaterial {
        base_color_texture: Some(textures.floor_texture.clone()),
        ..default()
    });

    // arch materials
    let arch_mesh = meshes.add(Mesh::from(Cuboid::new(0.7, 2.0, 0.5)));
    let sup_arch_mesh = meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5)));
    let arch_material = materials.add(StandardMaterial {
        base_color_texture: Some(textures.arch_texture.clone()),
        ..default()
    });

    (
        mesh_face,
        house_materials,
        branch_material,
        leaf_material,
        branch_mesh,
        leaf_mesh,
        wall_material,
        wall_mesh,
        floor_mesh,
        floor_material,
        arch_mesh,
        sup_arch_mesh,
        arch_material,
    )
}
