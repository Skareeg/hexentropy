use bevy::prelude::*;

pub const LEVEL_SIZE_X: usize = 128;
pub const LEVEL_SIZE_Y: usize = 128;
pub const LEVEL_SIZE_Z: usize = 8;
pub const LVL_S_C: usize = LEVEL_SIZE_X * LEVEL_SIZE_Y * LEVEL_SIZE_Z;

#[derive(Component)]
pub struct Level {
    pub tiles: [Entity; LVL_S_C],
}

#[derive(Component)]
pub struct Opaque;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct GenRoom;

pub fn gen_dungeon_floor(mut commands: Commands) {
}