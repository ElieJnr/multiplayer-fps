use bevy::math::primitives::{Cylinder, Sphere};
use bevy::prelude::*;
use serde_json::Value;
use std::f32::consts::PI;

use crate::graphics::map::MazeState;

#[derive(Component)]
pub struct ProceduralTree;

#[derive(Component)]
pub struct Sky;

#[derive(Component)]
pub struct MinimapCamera;

#[derive(Component)]
pub struct Arch;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Default, Resource)]
pub struct CameraState {
    is_top_view: bool,
}

pub struct Branch(Transform, Option<usize>, bool);

#[derive(Debug, Resource)]
pub struct TreeParams {
    children: u8,
    levels: u8,
    child_translation_factor: f32,
    angle_from_parent_branch: f32,
    child_scale: f32,
    base_radius: f32,
    leaf_radius: f32,
}

struct Textures {
    wall_texture: Handle<Image>,
    arch_texture: Handle<Image>,
    floor_texture: Handle<Image>,
    // wall_desert_texture: Handle<Image>,
    sky_texture: Handle<Image>,
    house_textures: HouseTextures,
}

struct HouseTextures {
    house_1: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_2: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_3: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_4: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_5: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_6: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_7: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
    house_8: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>),
}

struct HouseMaterials {
    house_1: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_2: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_3: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_4: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_5: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_6: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_7: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
    house_8: (Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>, Handle<StandardMaterial>),
}

impl Default for TreeParams {
    fn default() -> Self {
        Self {
            children: 3,
            levels: 5,
            child_translation_factor: 1.0,
            angle_from_parent_branch: 0.45,
            child_scale: 0.72,
            base_radius: 0.1,
            leaf_radius: 0.4,
        }
    }
}

// permet d'appeler les fonctions pour la creation de la scène
pub fn maze_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, params: Res<TreeParams>, mut maze_state: ResMut<MazeState>) {
    let map_data = include_str!("../maze.json");
    let map: Value = serde_json::from_str(map_data).unwrap();
    let maze = map["maze-1"].as_array().unwrap();
    let height = maze.len() as f32;
    let width = maze[0].as_array().unwrap().len() as f32;
    let textures = load_textures(&asset_server);
    commands.insert_resource(CameraState::default());
    let (mesh_face, house_materials, branch_material, leaf_material, branch_mesh, leaf_mesh, wall_material, wall_mesh, floor_mesh, floor_material, arch_mesh, sup_arch_mesh, arch_material) = initialize_materials(&mut meshes, &mut materials, &textures, &*params, width, height);
    create_surface(&mut commands, floor_mesh, floor_material, width, height);
    create_sky(&mut commands, &mut meshes, &mut materials, textures.sky_texture);
    create_lights(&mut commands, width, height);
    create_camera(&mut commands, Vec3::new(26.5, 1.0, 10.45), Vec3::ZERO, width, height);

    for (i, row) in maze.iter().enumerate() {
        let row = row.as_array().unwrap();
        for (j, cell) in row.iter().enumerate() {
            match cell.as_i64().unwrap() {
                1 => create_walls(&mut commands, wall_mesh.clone(), wall_material.clone(), i, j),
                2 => {
                    let arch = commands.spawn(SpatialBundle::default()).insert(Arch).id();
                    create_arch(&mut commands, &mut meshes, arch_mesh.clone(), sup_arch_mesh.clone(), arch_material.clone(), arch,  2.0,  1.0, i as f32, j as f32);
                }
                3 => create_procedural_tree(&mut commands, &mut meshes, &mut materials, &params, Vec3::new(j as f32, 0.0, i as f32), branch_material.clone(), leaf_material.clone(), branch_mesh.clone(),leaf_mesh.clone()),
                4 => create_house(&mut commands, mesh_face.clone(), &house_materials, Vec3::new(j as f32, 0.0, i as f32)),
                5 => create_walls(&mut commands, wall_mesh.clone(), wall_material.clone(), i, j),
                _ => {}
            }
        }
    }
    maze_state.is_ready = true;
}

// permet de créer un arbre procedural en utilisant les paramètres spécifiés
fn create_procedural_tree(commands: &mut Commands, _meshes: &mut ResMut<Assets<Mesh>>, _materials: &mut ResMut<Assets<StandardMaterial>>, params: &TreeParams, position: Vec3, branch_material: Handle<StandardMaterial>, leaf_material: Handle<StandardMaterial>, branch_mesh: Handle<Mesh>, leaf_mesh: Handle<Mesh>,
) {
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
fn generate_tree(params: &TreeParams) -> Vec<Branch> {
    let base = Transform::default();
    let mut ret: Vec<Branch> = Vec::new();
    ret.push(Branch(base, None, false));
    generate_branches(params, 1, 0, &mut ret);
    ret
}

// permet de générer les branches
fn generate_branches(params: &TreeParams, level: u8, parent_idx: usize, all: &mut Vec<Branch>) {
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
fn generate_leaves(parent_idx: usize, all: &mut Vec<Branch>) {
    let mut child_transform = Transform::IDENTITY;
    child_transform = child_transform.with_translation(*child_transform.local_y());
    all.push(Branch(child_transform, Some(parent_idx), true));
}

// permet de créer le sol en utilisant une taille de 0.1 et une texture de sol
fn create_surface(commands: &mut Commands, floor_mesh: Handle<Mesh>, floor_material: Handle<StandardMaterial>, width: f32, height: f32) {
    commands.spawn(PbrBundle {
        mesh: floor_mesh.clone(),
        material: floor_material.clone(),
        transform: Transform::from_xyz(width / 2.0, 0.0, height / 2.0),
        ..default()
    });
}

// permet de créer les murs en utilisant une taille de 1.0, une largeur de 2.0 et une hauteur de 1.0 tous en les positionnant correctement aux coordonnées i et j
fn create_walls(commands: &mut Commands, wall_mesh: Handle<Mesh>, wall_material: Handle<StandardMaterial>, i: usize, j: usize) {
    commands.spawn(PbrBundle {
        mesh: wall_mesh.clone(),
        material: wall_material.clone(),
        transform: Transform::from_xyz(j as f32, 1.0, i as f32),
        ..default()
    });
}

// permet de créer la caméra en la positionnant à la position donnée et en la regardant vers la cible donnée
fn create_camera(commands: &mut Commands, _position: Vec3, _target: Vec3, width: f32, height: f32) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(width / 2.0, 1.0, height - 2.0)
                .looking_at(Vec3::new(width / 2.0, 1.7, 0.0), Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));
}

// permet de créer les lumières en les positionnant au centre de la scène
fn create_lights(commands: &mut Commands, width: f32, height: f32) {
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
fn create_sky(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, sky_texture: Handle<Image>) {
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

// permet de controller la caméra en utilisant les touches du clavier
pub fn camera_controller(time: Res<Time>, keyboard_input: ResMut<'_, ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<PlayerCamera>>, mut camera_state: ResMut<CameraState>) {
    // let mut camera_transform = query.single_mut();

    if let Ok(mut camera_transform) = query.get_single_mut() {
        let speed = 5.0;
        let rotation_speed = 2.0;
        let ground_level = 1.0;

        if keyboard_input.just_pressed(KeyCode::KeyV) {
            camera_state.is_top_view = !camera_state.is_top_view;
            if camera_state.is_top_view {
                // vue de haut
                camera_transform.translation = Vec3::new(
                    camera_transform.translation.x,
                    50.0,
                    camera_transform.translation.z,
                );
                camera_transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
            } else {
                // vue FPS
                camera_transform.translation.y = ground_level;
                camera_transform.rotation = Quat::IDENTITY;
            }
        }

        if !camera_state.is_top_view {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                let forward = camera_transform.forward();
                camera_transform.translation += forward * speed * time.delta_seconds();
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                let forward = camera_transform.forward();
                camera_transform.translation -= forward * speed * time.delta_seconds();
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                camera_transform.rotate_y(rotation_speed * time.delta_seconds());
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                camera_transform.rotate_y(-rotation_speed * time.delta_seconds());
            }
            camera_transform.translation.y = ground_level;
        }
    } 
}

// permet de créer un arc en utilisant des piliers et un arc supérieur
fn create_arch(commands: &mut Commands, _meshes: &mut ResMut<Assets<Mesh>>, arch_mesh: Handle<Mesh>, sup_arch_mesh: Handle<Mesh>, arch_material: Handle<StandardMaterial>, arch: Entity, pillar_height: f32, arch_radius: f32, i: f32, j: f32) {

    if i == 29.0 && j == 22.0 {
        commands.entity(arch).insert(
            Transform::from_xyz(j - 0.26, 0.0, i + 0.5)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
        );
    } else if i == 30.0 && j == 22.0 {
        commands.entity(arch).insert(
            Transform::from_xyz(j + 0.26, 0.0, i - 0.5)
                .with_rotation(Quat::from_rotation_y(PI / 2.0)),
        );
    } else if i == 19.0 && j == 35.0 {
        commands.entity(arch).insert(
            Transform::from_xyz(j - 0.5, 0.0, i + 0.28)
                .with_rotation(Quat::from_rotation_y(PI * 2.0)),
        );
    } else {
        commands.entity(arch).insert(
            Transform::from_xyz(j - 0.5, 0.0, i + 0.28)
                .with_rotation(Quat::from_rotation_y(PI * 2.0)),
        );
    }


    // Piliers
    for x in [-arch_radius, arch_radius] {
        commands
            .spawn(PbrBundle {
                mesh: arch_mesh.clone(),
                material: arch_material.clone(),
                transform: Transform::from_xyz(x, pillar_height / 2.0, 0.0),
                ..default()
            })
            .set_parent(arch);
    }

    // Arc supérieur
    let segments = 16;
    for i in 0..segments {
        let angle = PI * (i as f32) / (segments - 1) as f32;
        let x = angle.cos() * arch_radius;
        let y = angle.sin() * arch_radius + pillar_height;

        commands
            .spawn(PbrBundle {
                mesh: sup_arch_mesh.clone(),
                material: arch_material.clone(),
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_rotation(Quat::from_rotation_z(angle)),
                ..default()
            })
            .set_parent(arch);
    }
}

// permet de créer une maison en utilisant 4 facades de mur
fn create_house(commands: &mut Commands, meshes: Handle<Mesh>, house_materials: &HouseMaterials, position: Vec3) {
    let wall_size = Vec3::new(3.0, 2.5, 0.1);

    let (front_material, back_material, left_material, right_material) = if position.x == 7.0 && position.z == 29.0 {
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
        .spawn(SpatialBundle {
            transform: Transform::from_translation(Vec3::new(
                position.x,
                wall_size.y / 8.6,
                position.z,
            )),
            ..default()
        })
        .id();

    // Front face
    commands
        .spawn(PbrBundle {
            mesh: meshes.clone(),
            material: front_material,
            transform: Transform::from_xyz(0.0, 1.0, 1.5),
            ..default()
        })
        .set_parent(house);

    // Back face
    commands
        .spawn(PbrBundle {
            mesh: meshes.clone(),
            material: back_material,
            transform: Transform::from_xyz(0.0, 1.0, -1.5),
            ..default()
        })
        .set_parent(house);

    // Left face
    commands
        .spawn(PbrBundle {
            mesh: meshes.clone(),
            material: left_material,
            transform: Transform::from_xyz(-1.5, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            ..default()
        })
        .set_parent(house);

    // Right face
    commands
        .spawn(PbrBundle {
            mesh: meshes.clone(),
            material: right_material,
            transform: Transform::from_xyz(1.5, 1.0, 0.0)
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
            ..default()
        })
        .set_parent(house);
}

// permet de charger les textures
fn load_textures(asset_server: &Res<AssetServer>) -> Textures {
    Textures {
        wall_texture: asset_server.load("textures/wall_2.png"),
        arch_texture: asset_server.load("textures/wall.png"),
        floor_texture: asset_server.load("textures/floor_sand.png"),
        // wall_desert_texture: asset_server.load("textures/wall_desert.png"),
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

fn initialize_materials(meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, textures: &Textures, params: &TreeParams, width: f32, height: f32) -> (Handle<Mesh>, HouseMaterials, Handle<StandardMaterial>,
        Handle<StandardMaterial>,
        Handle<Mesh>,
        Handle<Mesh>,
        Handle<StandardMaterial>,
        Handle<Mesh>,
        Handle<Mesh>,
        Handle<StandardMaterial>,
        Handle<Mesh>,
        Handle<Mesh>,
        Handle<StandardMaterial>,
    ) {
        // house materials
        let wall_size = Vec3::new(3.0, 2.5, 0.1);
        let mesh_face = meshes.add(Mesh::from(Cuboid::new(wall_size.x, wall_size.y, wall_size.z)));
        let house_materials = HouseMaterials {
            house_1: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_1.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_1.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_1.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_1.3.clone()),
                    ..default()
                }),
            ),
            house_2: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_2.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_2.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_2.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_2.3.clone()),
                    ..default()
                }),
            ),
            house_3: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_3.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_3.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_3.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_3.3.clone()),
                    ..default()
                }),
            ),
            house_4: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_4.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_4.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_4.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_4.3.clone()),
                    ..default()
                }),
            ),
            house_5: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_5.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_5.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_5.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_5.3.clone()),
                    ..default()
                }),
            ),
            house_6: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_6.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_6.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_6.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_6.3.clone()),
                    ..default()
                }),
            ),
            house_7: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_7.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_7.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_7.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_7.3.clone()),
                    ..default()
                }),
            ),
            house_8: (
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_8.0.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_8.1.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_8.2.clone()),
                    ..default()
                }),
                materials.add(StandardMaterial {
                    base_color_texture: Some(textures.house_textures.house_8.3.clone()),
                    ..default()
                }),
            ),
        };

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
