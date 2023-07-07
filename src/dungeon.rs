use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBody, Collider};
use ndarray::Array3;
use rltk::FastNoise;

use crate::{GameAssets, tileset_1bit::TileSet1Bit};

pub const LEVEL_SIZE_X: usize = 32;
pub const LEVEL_SIZE_Y: usize = 8;
pub const LEVEL_SIZE_Z: usize = 32;
pub const LVL_S_C: usize = LEVEL_SIZE_X * LEVEL_SIZE_Y * LEVEL_SIZE_Z;

pub struct LvlPlugin;

impl Plugin for LvlPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LvlState>()
            .init_resource::<Level>()
            .init_resource::<MaterialTypes>()
            .add_system(spawn_tile)
            .add_system(destroy_tile)
            .add_system(destroy_tile_rect)
            .add_system(gen_dungeon_init);
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
    pub tile_scale: f32,
}

impl Default for Level {
    fn default() -> Self {
        let size = [LEVEL_SIZE_X, LEVEL_SIZE_Y, LEVEL_SIZE_Z];
        Self {
            tiles: Array3::from_shape_simple_fn(size, || None),
            size,
            tile_scale: 0.5,
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

impl FromWorld for MaterialTypes {
    fn from_world(world: &mut World) -> Self {
        let mesh = {
            let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
            meshes.add(Mesh::from(shape::Cube { size: 0.5 }))
        };
        let (stone_mat, dirt_mat, wood_mat) = {
            let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
            (materials.add(Color::GRAY.into()), materials.add(Color::BEIGE.into()), materials.add(Color::BISQUE.into()))
        };
        Self {
            map: HashMap::from([
                (0, MaterialType { id: 0, name: "Stone".to_owned(), mesh: mesh.clone(), material: stone_mat }),
                (1, MaterialType { id: 1, name: "Dirt".to_owned(), mesh: mesh.clone(), material: dirt_mat }),
                (2, MaterialType { id: 2, name: "Wood".to_owned(), mesh: mesh.clone(), material: wood_mat }),
            ]),
        }
    }
}

impl MaterialTypes {
    pub fn get_mat(&self, name: &str) -> Option<&MaterialType> {
        self.map.values().find(|v| v.name == name)
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
    pub body: RigidBody,
    pub colllider: Collider,
}

impl TileBundle {
    pub fn new(position: [usize; 3], mat: &MaterialType, tile_scale: f32) -> Self {
        let world_pos = [position[0] as f32 * tile_scale, position[1] as f32 * tile_scale, position[2] as f32 * tile_scale];
        Self { tile: Tile, active: Active, opaque: Opaque, solid: Solid,
            pbr: PbrBundle {
                mesh: mat.mesh.clone(),
                material: mat.material.clone(),
                transform: Transform::from_translation(world_pos.into()),
                ..default()
            },
            mat: Material {
                id: mat.id,
            },
            body: RigidBody::Fixed,
            colllider: Collider::cuboid(tile_scale, tile_scale, tile_scale),
        }
    }
}

#[derive(Component)]
pub struct CmdSpawnTile {
    pub pos: [usize; 3],
    pub mat: usize,
}

fn spawn_tile(mut commands: Commands, spawns: Query<(Entity, &CmdSpawnTile)>, mut lvl: ResMut<Level>, mats: Res<MaterialTypes>) {
    for (ent, spawn) in &spawns {
        if lvl.tiles[spawn.pos].is_none() {
            let material_type = &mats.map[&spawn.mat];
            lvl.tiles[spawn.pos] = Some(commands.spawn(TileBundle::new(spawn.pos, material_type, lvl.tile_scale)).id());
        }
        commands.entity(ent).despawn();
    }
}

#[derive(Component)]
pub struct CmdDestroyTile {
    pub pos: [usize; 3],
}

fn destroy_tile(mut commands: Commands, destroys: Query<(Entity, &CmdDestroyTile)>, mut lvl: ResMut<Level>) {
    for (ent, destroy) in &destroys {
        if let Some(tile_entity) = lvl.tiles[destroy.pos] {
            commands.entity(tile_entity).despawn();
            lvl.tiles[destroy.pos] = None;
        }
        commands.entity(ent).despawn();
    }
}

#[derive(Component)]
pub struct CmdDestroyTileRect {
    pub min: [usize; 3],
    pub max: [usize; 3],
}

fn destroy_tile_rect(mut commands: Commands, destroys: Query<(Entity, &CmdDestroyTileRect)>, mut lvl: ResMut<Level>) {
    for (ent, destroy) in &destroys {
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
        commands.entity(ent).despawn();
    }
}

#[derive(Component)]
pub struct CmdLvlInit {
    pub seed: u64,
}

pub fn gen_dungeon_init(mut commands: Commands, inits: Query<(Entity, &CmdLvlInit)>, mut lvl: ResMut<Level>, mats: Res<MaterialTypes>) {
    for (ent, init) in &inits {
        let mut noise = FastNoise::seeded(init.seed);
        noise.set_noise_type(rltk::NoiseType::Perlin);
        noise.set_frequency(0.1);
        for x in 0..lvl.size[0] {
            for y in 0..lvl.size[1] {
                for z in 0..lvl.size[2] {
                    let pos = [x,y,z];
                    let stone = mats.get_mat("Stone").unwrap();
                    let dirt = mats.get_mat("Dirt").unwrap();
                    let wood = mats.get_mat("Wood").unwrap();
                    let val = noise.get_noise3d(x as f32, y as f32, z as f32) / 2.0 + 0.5;
                    let mut t = stone;
                    if val < 0.05 {
                        t = wood;
                    } else if val < 0.25 {
                        t = dirt;
                    }
                    lvl.tiles[pos] = Some(commands.spawn(
                        TileBundle::new(pos, t, lvl.tile_scale)
                    ).id());
                }
            }
        }
        commands.entity(ent).despawn();
        println!("Dungeon generated default materials.")
    }
}