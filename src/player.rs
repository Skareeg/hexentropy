use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use lerp::Lerp;

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
    mut players: Query<(&mut ExternalImpulse, &PlayerInput)>
) {
    for (mut impulse, input) in &mut players {
        match &input.movement {
            Some(pim) => {
                let pulse = Vec3::new(pim.x, pim.y, 0.) * 2000.0;
                impulse.impulse += pulse;
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
                transform.translation.x = transform.translation.x.lerp(track_transform.translation.x, 0.05 * time.elapsed_seconds());
                transform.translation.y = transform.translation.y.lerp(track_transform.translation.y, 0.05 * time.elapsed_seconds());
            }
            _ => {}
        }
    }
}