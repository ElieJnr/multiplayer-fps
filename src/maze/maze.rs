use super::models::*;
// use super::player_simulation::create_player;
use super::textures::*;
use crate::player::*;
use bevy::prelude::*;
use model::PlayerAnimations;
use model::PreloadedPlayerAnimations;
use player::create_players;
#[derive(Default, Resource)]
pub struct PosStruct {
    pub position: Vec3,
}
pub fn setup_crosshair(mut commands: Commands, asset_server: Res<AssetServer>) {
    let crosshair_texture = asset_server.load("textures/Crosshair.png");
    commands.spawn(ImageBundle {
        image: UiImage::from(crosshair_texture),
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            margin: UiRect {
                top: Val::Px(-15.0),
                left: Val::Px(-25.0),
                ..Default::default()
            },
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            ..Default::default()
        },
        ..default()
    });
}

pub fn maze_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    params: Res<TreeParams>,
    mut maze_state: ResMut<MazeState>,
    maze: Res<Maze>,
    preloaded_animations: Res<PreloadedPlayerAnimations>,
    player_animations: Res<PlayerAnimations>,
    pos_struct: Res<PosStruct>,
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
    create_players(
        &mut commands,
        preloaded_animations,
        player_animations,
        pos_struct.position,
    );
    maze_state.is_ready = true;
}
pub fn calculate_maze_dimensions(maze: &Vec<Vec<i32>>) -> (f32, f32) {
    let height = maze.len();
    let width = maze[0].len();
    (height as f32, width as f32)
}
