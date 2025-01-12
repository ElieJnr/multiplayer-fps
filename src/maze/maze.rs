use bevy::prelude::*;
use super::minimap::MazeState;
use super::textures::*;
use super::models::*;
use super::player_simulation::*;
use serde_json::Value;

pub fn maze_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, params: Res<TreeParams>, mut maze_state: ResMut<MazeState>) {
    let map_data = include_str!("../maze/maze.json");
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
    // create_camera(&mut commands, Vec3::new(26.5, 1.0, 10.45), Vec3::ZERO, width, height);

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
    
    create_player(&mut commands, &mut meshes, &mut materials, width, height);
    maze_state.is_ready = true;
}