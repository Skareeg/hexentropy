use bevy::prelude::*;
use lerp::Lerp;

use crate::character::CharacterMovement;

#[derive(Component)]
pub struct Player {
    pub id: u64,
}
#[derive(Component)]
pub struct NetLocal;
#[derive(Component)]
pub struct Remote {
    pub id: u64,
}

pub enum PlayerInputMove {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement: Option<Vec2>,
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut inputs: Query<&mut PlayerInput, With<NetLocal>>
) {
    for mut input in &mut inputs {
        input.movement = None;
        let mut mv = Vec2::ZERO;
        if keys.pressed(KeyCode::W) {
            mv.y += 1.0;
        }
        if keys.pressed(KeyCode::S) {
            mv.y -= 1.0;
        }
        if keys.pressed(KeyCode::A) {
            mv.x -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            mv.x += 1.0;
        }
        if mv != Vec2::ZERO {
            input.movement = Some(mv);
        }
    }
}

pub fn player_movement(
    mut players: Query<(&PlayerInput, &mut CharacterMovement)>
) {
    for (input, mut char) in &mut players {
        match &input.movement {
            Some(pim) => {
                char.requested = Some(Vec3::new(pim.x, pim.y, 0.));
            }
            None => {}
        }
    }
}

#[derive(Component)]
pub struct CameraTrackEntity {
    pub ent: Entity,
}

pub fn camera_track_entity(
    mut cameras: Query<(&mut Transform, &CameraTrackEntity)>,
    ents: Query<&Transform, Without<CameraTrackEntity>>,
    time: Res<Time>
)
{
    for (mut transform, track) in &mut cameras {
        match ents.get(track.ent) {
            Ok(track_transform) => {
                transform.translation.x = transform.translation.x.lerp(track_transform.translation.x, 0.5 * time.delta_seconds());
                transform.translation.y = transform.translation.y.lerp(track_transform.translation.y, 0.5 * time.delta_seconds());
            }
            _ => {}
        }
    }
}