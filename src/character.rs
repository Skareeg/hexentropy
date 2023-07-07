use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(char_accel_movement_aim)
            .add_system(char_accel_movement_update);
    }
}

#[derive(Component, Default)]
pub struct CharacterMovement {
    pub requested: Option<Vec3>,
    pub aim_requested: Option<Vec2>,
    pub acceleration: f32,
    pub dampening: f32,
    pub velocity: Vec3,
    pub max_speed: f32,
    pub min_threshold: f32,
    pub grounded: bool,
}

#[derive(Component, Default)]
pub struct CharacterHead;

pub fn char_accel_movement_aim(
    mut characters: Query<(&mut Transform, &mut CharacterMovement), Without<CharacterHead>>,
    mut heads: Query<(&Parent, &mut Transform), With<CharacterHead>>,
    time: Res<Time>
) {
    for (parent, mut head_transform) in &mut heads {
        if let Ok((_, movement)) = characters.get(parent.get()) {
            if let Some(aim) = movement.aim_requested {
                println!("char_accel_movement_aim (head)");
                head_transform.rotate_local_x(aim.y * time.delta_seconds());
            }
        }
    }
    for (mut transform, mut movement) in &mut characters {
        if let Some(aim) = movement.aim_requested {
            println!("char_accel_movement_aim (body)");
            transform.rotate_z(aim.x * time.delta_seconds());
            movement.aim_requested = None;
        }
    }
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
            println!("char_accel_movement_update");
            movement.requested = None;
            character.translation = Some(movement.velocity);
        }
    }
}