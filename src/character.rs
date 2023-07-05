use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

#[derive(Component, Default)]
pub struct CharacterMovement {
    pub requested: Option<Vec3>,
    pub acceleration: f32,
    pub dampening: f32,
    pub velocity: Vec3,
    pub max_speed: f32,
    pub min_threshold: f32,
    pub grounded: bool,
}

pub fn char_accel_movement_update(
    mut characters: Query<(&mut KinematicCharacterController, &mut CharacterMovement)>,
    time: Res<Time>
) {
    for (mut character, mut movement) in &mut characters {
        if let Some(acceleration) = movement.requested {
            movement.velocity = movement.velocity + (acceleration * time.delta_seconds());
            movement.requested = None;
        } else {
            if movement.velocity != Vec3::ZERO {
                movement.velocity = movement.velocity.lerp(movement.velocity / (1.0 + movement.dampening), time.delta_seconds());
                if movement.velocity.length() < movement.min_threshold {
                    movement.velocity = Vec3::ZERO;
                }
            }
        }
        if movement.velocity != Vec3::ZERO {
            movement.requested = None;
            character.translation = Some(movement.velocity);
        }
    }
}