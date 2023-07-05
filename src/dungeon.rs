use bevy::prelude::*;
use ndarray::Array3;

use crate::{GameAssets, tileset_1bit::TileSet1Bit};

pub const LEVEL_SIZE_X: usize = 128;
pub const LEVEL_SIZE_Y: usize = 128;
pub const LEVEL_SIZE_Z: usize = 8;
pub const LVL_S_C: usize = LEVEL_SIZE_X * LEVEL_SIZE_Y * LEVEL_SIZE_Z;

#[derive(Component)]
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

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Opaque;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct GenRoom;

#[derive(Bundle)]
pub struct WallBundle {
    pub opaque: Opaque,
    pub solid: Solid,
    pub texture_bundle: SpriteSheetBundle,
    // pub texture: Handle<TextureAtlas>,
    // pub sprite: TextureAtlasSprite,
    // pub transform: Transform,
    // pub global: GlobalTransform,
}

impl WallBundle {
    pub fn new(position: [usize; 3], atlas: Handle<TextureAtlas>, tile: TileSet1Bit) -> Self {
        Self { opaque: Opaque, solid: Solid, texture_bundle: SpriteSheetBundle {
            texture_atlas: atlas,
            sprite: TextureAtlasSprite::new(tile as usize),
            transform: Transform::from_xyz(position[0] as f32 * 16.0, position[1] as f32 * 16.0, position[2] as f32 * 16.0),
            ..default()
        } }
    }
}

pub fn gen_dungeon_floor(mut commands: Commands, assets: Res<GameAssets>, mut levels: Query<(Entity, &mut Level), With<GenRoom>>) {
    for (ent, mut level) in &mut levels {
        let i1 = 48usize;
        let i2 = 78usize;
        let j1 = 48usize;
        let j2 = 70usize;
        let k = 4usize;
        // Create a small test room to play in.
        for i in i1..=i2 {
            let pos1 = [i, j1, k];
            let pos2 = [i, j2, k];
            level.tiles[pos1] = Some(commands.spawn(WallBundle::new(pos1, assets.atlas1.clone(), TileSet1Bit::BlockX)).id());
            level.tiles[pos2] = Some(commands.spawn(WallBundle::new(pos2, assets.atlas1.clone(), TileSet1Bit::BlockX)).id());
        }
        for j in j1+1..j2 {
            let pos1 = [i1, j, k];
            let pos2 = [i2, j, k];
            level.tiles[pos1] = Some(commands.spawn(WallBundle::new(pos1, assets.atlas1.clone(), TileSet1Bit::BlockX)).id());
            level.tiles[pos2] = Some(commands.spawn(WallBundle::new(pos2, assets.atlas1.clone(), TileSet1Bit::BlockX)).id());
        }
        commands.entity(ent).remove::<GenRoom>();
        println!("Dungeon generated test room.")
    }
}