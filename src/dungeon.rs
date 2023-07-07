use std::collections::HashMap;

use bevy::prelude::*;
use ndarray::Array3;

use crate::{GameAssets, tileset_1bit::TileSet1Bit};

pub const LEVEL_SIZE_X: usize = 128;
pub const LEVEL_SIZE_Y: usize = 128;
pub const LEVEL_SIZE_Z: usize = 8;
pub const LVL_S_C: usize = LEVEL_SIZE_X * LEVEL_SIZE_Y * LEVEL_SIZE_Z;

pub struct LvlPlugin;

impl Plugin for LvlPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LvlState>();
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum LvlState {
    #[default]
    Generate,
    Ready,
    Clean,
}

#[derive(Resource)]
pub struct Level {
    pub tiles: Array3<Option<Entity>>,
    pub size: [usize; 3],
}

impl Default for Level {
    fn default() -> Self {
        let size = [LEVEL_SIZE_X, LEVEL_SIZE_Y, LEVEL_SIZE_Z];
        Self {
            tiles: Array3::from_shape_simple_fn(size, || None),
            size,
        }
    }
}

pub struct MaterialType {
    pub id: usize,
    pub name: String,
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct MaterialTypes {
    pub map: HashMap<usize, MaterialType>,
}

impl MaterialTypes {
    fn new(mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        let mesh = meshes.add(
            Mesh::from(shape::Cube { size: 0.5 })
        );
        Self {
            map: HashMap::from([
                (0, MaterialType { id: 0, name: "Stone".to_owned(), mesh: mesh.clone(), material: materials.add(Color::GRAY.into()) }),
                (1, MaterialType { id: 1, name: "Dirt".to_owned(), mesh: mesh.clone(), material: materials.add(Color::BEIGE.into()) }),
                (2, MaterialType { id: 2, name: "Wood".to_owned(), mesh: mesh.clone(), material: materials.add(Color::BISQUE.into()) }),
            ]),
        }
    }
}

#[derive(Component)]
pub struct Active;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Material {
    pub id: usize,
}

#[derive(Component)]
pub struct Opaque;

#[derive(Component)]
pub struct Solid;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub active: Active,
    pub opaque: Opaque,
    pub solid: Solid,
    pub pbr: PbrBundle,
    pub mat: Material,
}

impl TileBundle {
    pub fn new(position: [usize; 3], mat: &MaterialType) -> Self {
        Self { tile: Tile, active: Active, opaque: Opaque, solid: Solid,
            pbr: PbrBundle {
                mesh: mat.mesh.clone(),
                material: mat.material.clone(),
                transform: Transform::from_xyz(position[0] as f32, position[1] as f32, position[2] as f32),
                ..default()
            },
            mat: Material {
                id: mat.id,
            },
        }
    }
}

#[derive(Component)]
pub struct CmdSpawnTile {
    pub pos: [usize; 3],
    pub mat: usize,
}

fn spawn_tile(mut commands: Commands, spawns: Query<&CmdSpawnTile>, mut lvl: ResMut<Level>, mats: Res<MaterialTypes>) {
    for spawn in &spawns {
        if lvl.tiles[spawn.pos].is_none() {
            let material_type = &mats.map[&spawn.mat];
            lvl.tiles[spawn.pos] = Some(commands.spawn(TileBundle::new(spawn.pos, material_type)).id());
        }
    }
}

#[derive(Component)]
pub struct CmdDestroyTile {
    pub pos: [usize; 3],
}

fn destroy_tile(mut commands: Commands, destroys: Query<&CmdDestroyTile>, mut lvl: ResMut<Level>) {
    for destroy in &destroys {
        if let Some(tile_entity) = lvl.tiles[destroy.pos] {
            commands.entity(tile_entity).despawn();
            lvl.tiles[destroy.pos] = None;
        }
    }
}

#[derive(Component)]
pub struct CmdDestroyTileRect {
    pub min: [usize; 3],
    pub max: [usize; 3],
}

fn destroy_tile_rect(mut commands: Commands, destroys: Query<&CmdDestroyTileRect>, mut lvl: ResMut<Level>) {
    for destroy in &destroys {
        for x in destroy.min[0]..=destroy.max[0] {
            for y in destroy.min[1]..=destroy.max[1] {
                for z in destroy.min[2]..=destroy.max[2] {
                    let pos = [x,y,z];
                    if let Some(tile_entity) = lvl.tiles[pos] {
                        commands.entity(tile_entity).despawn();
                        lvl.tiles[pos] = None;
                    }
                }
            }
        }
    }
}

pub fn gen_dungeon_init(mut commands: Commands, mut lvl: ResMut<Level>) {
    for x in 0..=lvl.size[0] {
        for y in 0..=lvl.size[1] {
            for z in 0..=lvl.size[2] {
            }
        }
    }
    println!("Dungeon generated test room.")
}

pub fn gen_dungeon_init_start_room(mut commands: Commands, mut lvl: ResMut<Level>) {
    let i1 = 48usize;
    let i2 = 78usize;
    let j1 = 48usize;
    let j2 = 70usize;
    let k1 = 0usize;
    let k2 = 4usize;
    println!("Dungeon generated test room.")
}