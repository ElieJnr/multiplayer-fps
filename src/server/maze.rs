use bevy::prelude::*;
use serde_json::Value;
use std::f32::consts::PI;
use bevy::math::primitives::{Sphere, Cylinder};

#[derive(Component)]
struct ProceduralTree;

#[derive(Component)]
struct Sky;

#[derive(Component)]
struct Arch;

#[derive(Component)]
struct PlayerCamera;

struct Branch(Transform, Option<usize>, bool);

#[derive(Debug, Resource)]
struct TreeParams {
    children: u8,
    levels: u8,
    child_translation_factor: f32,
    angle_from_parent_branch: f32,
    child_scale: f32,
    base_radius: f32,
    leaf_radius: f32,
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

pub fn setup_maze() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<TreeParams>()
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_sky, camera_controller).chain())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, params: Res<TreeParams>) {
    let map_data = include_str!("../maze.json");
    let map: Value = serde_json::from_str(map_data).unwrap();
    let maze = map["maze-1"].as_array().unwrap();

    let floor_texture = asset_server.load("sand.png");
    let wall_texture = asset_server.load("wall_2.png");
    let sky_texture = asset_server.load("cloud_2.png");
    let arch_texture = asset_server.load("wall.png");
    let house_1_textures = (
        asset_server.load("house_1_front.png"),
        asset_server.load("house_1_back.png"),
        asset_server.load("house_1_left.png"),
        asset_server.load("house_1_right.png"),
    );
    let house_2_textures = (
        asset_server.load("house_2_back.png"),
        asset_server.load("house_2.png"),
        asset_server.load("house_2_left.png"),
        asset_server.load("house_2_right.png"),
    );
    let house_3_textures = (
        asset_server.load("house_3_front.png"),
        asset_server.load("house_3_back.png"),
        asset_server.load("house_3_left.png"),
        asset_server.load("house_3_right.png"),
    );
    let house_4_textures = (
        asset_server.load("house_4_back.png"),
        asset_server.load("house_4.png"),
        asset_server.load("house_4_left.png"),
        asset_server.load("house_4_right.png"),
    );


    let height = maze.len() as f32;
    let width = maze[0].as_array().unwrap().len() as f32;

    create_walls(&mut commands, &mut meshes, &mut materials, wall_texture, maze);
    create_sky(&mut commands, &mut meshes, &mut materials, sky_texture);
    create_surface(&mut commands, &mut meshes, &mut materials, floor_texture, width, height);
    create_lights(&mut commands, width, height);
    create_camera(&mut commands, Vec3::new(26.5, 1.0, 10.45), Vec3::ZERO, width, height);

    // create_house(&mut commands, &mut meshes, &mut materials, asset_server);

    let arch_material = materials.add(StandardMaterial {
        base_color_texture: Some(arch_texture),
        perceptual_roughness: 0.5,
        ..default()
    });

    let branch_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    let leaf_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.8, 0.3),
        ..default()
    });


    for (i, row) in maze.iter().enumerate() {
        let row = row.as_array().unwrap();
        for (j, cell) in row.iter().enumerate() {
            match cell.as_i64().unwrap() {
                2 => { // Arch
                    let arch = commands.spawn(SpatialBundle::default()).insert(Arch).id();
                    create_arch(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        arch_material.clone(),
                        arch,
                        0.7,
                        2.0,
                        0.5,
                        1.0,
                        i as f32,
                        j as f32
                    );
                }
                3 => { // Tree
                    create_procedural_tree(
                        &mut commands,
                        &mut meshes,
                        branch_material.clone(),
                        leaf_material.clone(),
                        &params,
                        Vec3::new(j as f32, 0.0, i as f32),
                    );
                }

                4 => { // House
                    create_house(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        house_1_textures.clone(),
                        house_2_textures.clone(),
                        house_3_textures.clone(),
                        house_4_textures.clone(),
                        Vec3::new(j as f32, 0.0, i as f32),
                    );
                }
                _ => {}
            }
        }
    }
}

fn create_procedural_tree(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, branch_material: Handle<StandardMaterial>, leaf_material: Handle<StandardMaterial>, params: &TreeParams, position: Vec3) {
    let tree = generate_tree(params);
    let mut entity_parent_indices: Vec<(Entity, Option<usize>)> = Vec::new();

    // Création des meshs réutilisables
    let branch_mesh = meshes.add(Cylinder {
        radius: params.base_radius,
        half_height: 0.5,
    });

    let leaf_mesh = meshes.add(Sphere {
        radius: params.leaf_radius,
    });

    // Créer l'entité racine de l'arbre avec la position spécifiée
    let root = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position),
            ..default()
        },
        ProceduralTree,
    )).id();

    // Générer toutes les branches et feuilles
    for branch in &tree {
        let entity_id = if branch.2 { // Si c'est une feuille
            commands.spawn(PbrBundle {
                mesh: leaf_mesh.clone(),
                material: leaf_material.clone(),
                transform: branch.0,
                ..default()
            }).id()
        } else { // Si c'est une branche
            commands.spawn(PbrBundle {
                mesh: branch_mesh.clone(),
                material: branch_material.clone(),
                transform: branch.0,
                ..default()
            }).id()
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

fn generate_tree(params: &TreeParams) -> Vec<Branch> {
    let base = Transform::default();
    let mut ret: Vec<Branch> = Vec::new();
    ret.push(Branch(base, None, false));
    generate_branches(params, 1, 0, &mut ret);
    ret
}

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

fn generate_leaves(parent_idx: usize, all: &mut Vec<Branch>) {
    let mut child_transform = Transform::IDENTITY;
    child_transform = child_transform.with_translation(*child_transform.local_y());
    all.push(Branch(child_transform, Some(parent_idx), true));
}

fn create_surface(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, floor_texture: Handle<Image>, width: f32, height: f32) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(width, 0.1, height))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(floor_texture),
            ..default()
        }),
        transform: Transform::from_xyz(width / 2.0, 0.0, height / 2.0),
        ..default()
    });
}

fn create_walls(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, wall_texture: Handle<Image>, maze: &[Value]) {
    for (i, row) in maze.iter().enumerate() {
        let row = row.as_array().unwrap();
        for (j, cell) in row.iter().enumerate() {
            if cell.as_i64().unwrap() == 1 {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 2.0, 1.0))),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(wall_texture.clone()),
                        ..default()
                    }),
                    transform: Transform::from_xyz(j as f32, 1.0, i as f32),
                    ..default()
                });
            }
        }
    }
}

fn create_camera(commands: &mut Commands, _position: Vec3, _target: Vec3, width: f32, height: f32) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(width / 2.0, 1.0, height - 2.0)
                .looking_at(Vec3::new(width / 2.0, 1.7, 0.0), Vec3::Y),
            ..default()
        },
        PlayerCamera
    ));
}

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

fn create_sky(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, sky_texture: Handle<Image>) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere {
                radius: 500.0,
            })),
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

fn rotate_sky(time: Res<Time>, mut query: Query<&mut Transform, With<Sky>>) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() * 0.05);
    }
}

fn camera_controller(time: Res<Time>, keyboard_input: ResMut<'_, ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<PlayerCamera>>) {
    let mut camera_transform = query.single_mut();
    
    let speed = 5.0;
    let rotation_speed = 2.0;

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
}

fn create_arch(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, _materials: &mut ResMut<Assets<StandardMaterial>>, arch_material: Handle<StandardMaterial>, arch: Entity, pillar_width: f32, pillar_height: f32, arch_depth: f32, arch_radius: f32, i: f32, j: f32) {
    
    println!("i{} j{}", i, j);

    if i == 27.0 && j == 21.0 {
        commands.entity(arch).insert(Transform::from_xyz(j-0.26, 0.0, i+0.5).with_rotation(Quat::from_rotation_y(PI / 2.0)));
    } else  if i == 28.0 && j == 21.0  {
        commands.entity(arch).insert(Transform::from_xyz(j+0.26, 0.0, i - 0.5).with_rotation(Quat::from_rotation_y(PI / 2.0)));
    } else if i == 19.0 && j == 35.0 {
        commands.entity(arch).insert(Transform::from_xyz(j-0.5, 0.0, i + 0.28).with_rotation(Quat::from_rotation_y(PI * 2.0)));
    } else {
        commands.entity(arch).insert(Transform::from_xyz(j-0.5, 0.0, i + 0.28).with_rotation(Quat::from_rotation_y(PI * 2.0)));
    }

    // Piliers
    for x in [-arch_radius, arch_radius] {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(
                pillar_width,
                pillar_height,
                arch_depth,
            )),
            material: arch_material.clone(),
            transform: Transform::from_xyz(x, pillar_height / 2.0, 0.0),
            ..default()
        }).set_parent(arch);
    }

    // Arc supérieur
    let segments = 16;
    for i in 0..segments {
        let angle = PI * (i as f32) / (segments - 1) as f32;
        let x = angle.cos() * arch_radius;
        let y = angle.sin() * arch_radius + pillar_height;
        
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(
                0.5,
                0.5,
                arch_depth,
            )),
            material: arch_material.clone(),
            transform: Transform::from_xyz(x, y, 0.0)
                .with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        }).set_parent(arch);
    }
}

fn create_house(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, house_1_textures: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>), house_2_textures: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>), house_3_textures: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>), house_4_textures: (Handle<Image>, Handle<Image>, Handle<Image>, Handle<Image>), position: Vec3) {
    println!("{}", position);

    let wall_size = Vec3::new(3.0, 2.5, 0.1);

    let (front_texture, back_texture, left_texture, right_texture) = if position.x == 7.0 && position.z == 27.0 {
        house_2_textures
    } else if position.x == 9.0 && position.z == 34.0  || position.x == 12.0 && position.z == 34.0 || position.x == 2.0 && position.z == 16.0 || position.x == 7.0 && position.z == 16.0 {
        house_3_textures
    } else if position.x == 4.0 && position.z == 32.0  {
        house_4_textures
    } else {
        house_1_textures
    };
    


    let house = commands.spawn(SpatialBundle {
        transform: Transform::from_translation(position),
        ..default()
    }).id();

     // Front face
     commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(wall_size.x, wall_size.y, wall_size.z))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(front_texture),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, 1.5),
        ..default()
    }).set_parent(house);

    // Back face
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(wall_size.x, wall_size.y, wall_size.z))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(back_texture),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 1.0, -1.5),
        ..default()
    }).set_parent(house);

    // Left face
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(wall_size.x, wall_size.y, wall_size.z))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(left_texture),
            ..default()
        }),
        transform: Transform::from_xyz(-1.5, 1.0, 0.0).with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
        ..default()
    }).set_parent(house);

    // Right face
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(wall_size.x, wall_size.y, wall_size.z))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(right_texture),
            ..default()
        }),
        transform: Transform::from_xyz(1.5, 1.0, 0.0).with_rotation(Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2)),
        ..default()
    }).set_parent(house);
}