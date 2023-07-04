use bevy::prelude::*;

#[derive(Component)]
pub struct Rigid;

#[derive(Component)]
pub struct PhysicsMaterial {
    pub mass: f32,
    pub bounce: f32,
}

#[derive(Component)]
pub struct BoxCollider {
    pub size: Vec3,
}

#[derive(Component)]
pub struct SphereCollider {
    pub size: f32,
}

#[derive(Component)]
pub struct Movement {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Dampener {
    pub dampening: f32,
}

#[derive(Component)]
pub struct Gravitation {
    pub gravity: Vec3,
}

pub fn simulate_dampening(mut bodies: Query<(&mut Movement, &Dampener)>, time: Res<Time>) {
    for (mut movement, dampener) in bodies.iter_mut() {
        movement.velocity = movement.velocity.lerp(movement.velocity * dampener.dampening, time.elapsed_seconds());
    }
}

pub fn simulate_movement(mut bodies: Query<(&mut Transform, &Movement)>, time: Res<Time>) {
    for (mut transform, movement) in bodies.iter_mut() {
        transform.translation += movement.velocity * time.delta_seconds();
    }
}

pub fn simulate_gravity(mut bodies: Query<(&mut Movement, &Gravitation)>, time: Res<Time>) {
    for (mut movement, gravitation) in bodies.iter_mut() {
        movement.velocity += gravitation.gravity * time.delta_seconds();
    }
}

pub fn simulate_collision_box_box(mut body1: Query<(Entity, &mut Transform, &mut Movement, &PhysicsMaterial, &BoxCollider)>, body2: Query<(Entity, &Transform, &Movement, &PhysicsMaterial, &BoxCollider)>) {
    for (e1, mut t1, mut v1, m1, c1) in body1.iter_mut() {
        for (e2, t2, v2, m2, c2) in body2.iter() {
            if e1 != e2 {
                let (hcx, hcy, hcz) = (c1.size.x / 2.0, c1.size.y / 2.0, c1.size.z / 2.0);
                let (ccx, ccy, ccz) = (c2.size.x / 2.0, c2.size.y / 2.0, c2.size.z / 2.0);
                // Test left collide.
                if t1.translation.x - hcx < t2.translation.x + ccx {
                }
            }
        }
    }
}