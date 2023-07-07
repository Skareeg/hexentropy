use bevy::{prelude::*, input::mouse::MouseMotion};
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
    Forward,
    Backward,
    Left,
    Right,
}

pub enum PlayerInputTurn {
    Yaw(f32),
    Pitch(f32),
}

#[derive(Component)]
pub struct PlayerInput {
    pub movement: Option<Vec2>,
    pub aiming: Option<Vec2>,
}

pub fn player_input_move(
    keys: Res<Input<KeyCode>>,
    mut inputs: Query<&mut PlayerInput, With<NetLocal>>
) {
    for mut input in &mut inputs {
        println!("player_input_move");
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
            println!("player_input_move");
            input.movement = Some(mv);
        }
    }
}
pub fn player_input_aim(
    mut motions: EventReader<MouseMotion>,
    mut inputs: Query<&mut PlayerInput, With<NetLocal>>
) {
    let mut summed_motions = Vec2::ZERO;
    for motion in motions.iter() {
        summed_motions += motion.delta;
        println!("delta");
    }
    for mut input in &mut inputs {
        println!("player_input_aim");
        if summed_motions != Vec2::ZERO {
            println!("player_input_aim");
            input.aiming = Some(summed_motions * 2f32.to_radians());
        }
    }
}

pub fn player_movement(
    mut players: Query<(&mut PlayerInput, &mut CharacterMovement, &Transform)>
) {
    for (mut input, mut char, transform) in &mut players {
        println!("player_movement");
        match &input.movement {
            Some(pim) => {
                println!("player_movement (moving)");
                char.requested = Some(transform.rotation * Vec3::new(pim.x, 0., pim.y));
            }
            None => {}
        }
        match &input.aiming {
            Some(aim) => {
                println!("player_movement (aiming)");
                char.aim_requested = Some(*aim);
            }
            None => {}
        }
        input.movement = None;
        input.aiming = None;
    }
}