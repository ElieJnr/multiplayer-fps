use super::models::*;
use super::player_simulation::*;
use super::textures::*;
use bevy::prelude::*;
use std::collections::HashMap;
use rand::Rng;

pub fn maze_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    params: Res<TreeParams>,
    mut maze_state: ResMut<MazeState>,
    maze: Res<Maze>
) {
    let map = &maze.maze_1;
    let (height, width) = calculate_maze_dimensions(map);
    let textures = load_textures(&asset_server);
    commands.insert_resource(CameraState::default());
    let (
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
    ) = initialize_materials(
        &mut meshes,
        &mut materials,
        &textures,
        &*params,
        width,
        height,
    );
    create_surface(&mut commands, floor_mesh, floor_material, width, height);
    create_sky(
        &mut commands,
        &mut meshes,
        &mut materials,
        textures.sky_texture,
    );
    create_lights(&mut commands, width, height);
    // create_camera(&mut commands, Vec3::new(26.5, 1.0, 10.45), Vec3::ZERO, width, height);

    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell {
                1 => create_walls(
                    &mut commands,
                    wall_mesh.clone(),
                    wall_material.clone(),
                    i,
                    j,
                ),
                2 => {
                    let arch = commands.spawn(SpatialBundle::default()).insert(Arch).id();
                    create_arch(
                        &mut commands,
                        &mut meshes,
                        arch_mesh.clone(),
                        sup_arch_mesh.clone(),
                        arch_material.clone(),
                        arch,
                        2.0,
                        1.0,
                        i as f32,
                        j as f32,
                    );
                }
                3 => create_procedural_tree(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &params,
                    Vec3::new(j as f32, 0.0, i as f32),
                    branch_material.clone(),
                    leaf_material.clone(),
                    branch_mesh.clone(),
                    leaf_mesh.clone(),
                ),
                4 => create_house(
                    &mut commands,
                    mesh_face.clone(),
                    &house_materials,
                    Vec3::new(j as f32, 0.0, i as f32),
                ),
                5 => create_walls(
                    &mut commands,
                    wall_mesh.clone(),
                    wall_material.clone(),
                    i,
                    j,
                ),
                _ => {}
            }
        }
    }

    let mut name_position:HashMap<String, Vec<f32>>=HashMap::new();
    name_position.insert("1".to_string(),vec![20.0, 1.0, 37.0]);
    name_position.insert("2".to_string(),vec![24.29, 1.0, 25.13]);
    name_position.insert("3".to_string(),vec![15.84, 1.0, 19.92]);
    name_position.insert("4".to_string(),vec![25.97, 1.0, 18.76]);
    name_position.insert("5".to_string(),vec![37.70, 1.0, 21.66]);
    name_position.insert("6".to_string(),vec![24.14, 1.0, 1.41]);
    name_position.insert("7".to_string(),vec![37.0, 1.0, 35.0]);
    name_position.insert("8".to_string(),vec![37.4, 1.0, 1.2]);
    name_position.insert("9".to_string(),vec![1.27, 1.0, 5.63]);
    name_position.insert("10".to_string(),vec![1.68, 1.0, 18.91]);

    let mut bool_position: HashMap<String,bool>=HashMap::new();
    bool_position.insert("1".to_string(),false);
    bool_position.insert("2".to_string(),false);
    bool_position.insert("3".to_string(),false);
    bool_position.insert("4".to_string(),false);
    bool_position.insert("5".to_string(),false);
    bool_position.insert("6".to_string(),false);
    bool_position.insert("7".to_string(),false);
    bool_position.insert("8".to_string(),false);
    bool_position.insert("9".to_string(),false);
    bool_position.insert("10".to_string(),false);

    let pos= choose_place(&mut name_position, &mut bool_position);

    println!("~~~~~~~~~~~~~~~~~~~{:?}~~~~~~~~~~~~~~~~~",pos);

    create_player(&mut commands, &mut meshes, &mut materials, width, height, pos);
    maze_state.is_ready = true;
}

pub fn calculate_maze_dimensions(maze: &Vec<Vec<i32>>) -> (f32, f32) {
    let height = maze.len();
    let width = maze[0].len();
    (height as f32, width as f32)
}

fn choose_place(
    name_position: &mut HashMap<String, Vec<f32>>,
    bool_position: &mut HashMap<String, bool>,
) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    
    // Générer un index aléatoire basé sur la longueur de name_position
    let random_number = rng.gen_range(0..name_position.len());
    
    // Obtenir la clé correspondant à l'index aléatoire
    let random_key = name_position.keys().nth(random_number).unwrap().clone();
    
    // Vérifier si la clé existe dans bool_position
    match bool_position.get_mut(&random_key) {
        Some(bool_value) => {
            // Vérifier si le lieu a déjà été choisi (i.e., si la valeur booléenne est true)
            if *bool_value {
                // Si déjà choisi, on rappelle la fonction pour essayer de choisir un autre lieu
                return choose_place(name_position, bool_position);
            } else {
                // Marquer le lieu comme choisi (mettre la valeur à true)
                *bool_value = true;
                
                // Retourner la valeur correspondante de name_position
                return name_position.get(&random_key).unwrap().clone();
            }
        },
        None => {
            // Si la clé n'existe pas dans bool_position, on rappelle la fonction pour essayer encore
            return choose_place(name_position, bool_position);
        }
    }
}