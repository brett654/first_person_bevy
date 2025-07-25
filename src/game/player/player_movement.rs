use bevy::prelude::*;
use super::{player::CameraController, player::MovementMode};

use bevy_rapier3d::prelude::*;

pub fn camera_movement_flying(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CameraController>>,
) {
    let Ok(mut transform) = query.single_mut() else { return };

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= *transform.right();
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let speed = 15.0;
    transform.translation += direction * speed * time.delta_secs();
}

pub fn camera_movement_with_collision(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Transform), With<CameraController>>,
) {
    let Ok((mut velocity, transform)) = query.single_mut() else { return };

    let mut wish_dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        wish_dir += *transform.forward();
    }
    if keys.pressed(KeyCode::KeyS) {
        wish_dir -= *transform.forward();
    }
    if keys.pressed(KeyCode::KeyA) {
        wish_dir -= *transform.right();
    }
    if keys.pressed(KeyCode::KeyD) {
        wish_dir += *transform.right();
    }
    if keys.just_pressed(KeyCode::Space) {
        velocity.linvel.y = 6.0; // tweak for jump height
    }

    // Project onto XZ plane
    wish_dir.y = 0.0;

    let delta = time.delta_secs();

    if wish_dir.length_squared() > 0.0 {
        wish_dir = wish_dir.normalize();
    }

    let acceleration = 30.0;
    let max_speed = 10.0;
    let friction = 8.0;

    // --- Save vertical velocity BEFORE changing linvel ---
    let mut vertical_velocity = velocity.linvel.y;

    // --- Horizontal movement (XZ only) ---
    let current_velocity_flat = Vec3::new(velocity.linvel.x, 0.0, velocity.linvel.z);
    let speed_in_dir = current_velocity_flat.dot(wish_dir);

    let add_speed = max_speed - speed_in_dir;
    if add_speed > 0.0 {
        let accel_speed = (acceleration * delta).min(add_speed);
        velocity.linvel += wish_dir * accel_speed;
    }

    // --- Jumping ---
    if keys.just_pressed(KeyCode::Space) {
        vertical_velocity = 6.0;
    }

    // --- Apply friction if no input ---
    if wish_dir == Vec3::ZERO {
        velocity.linvel.x -= velocity.linvel.x * friction * delta;
        velocity.linvel.z -= velocity.linvel.z * friction * delta;
    }

    // --- Restore vertical velocity ---
velocity.linvel.y = vertical_velocity;
}

pub fn switch_movement_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut MovementMode, &mut GravityScale), With<CameraController>>,
) {
    if keys.just_pressed(KeyCode::KeyF) {
        let Ok((mut mode, mut gravity)) = query.single_mut() else { return };

        *mode = match *mode {
            MovementMode::Flying => {
                *gravity = GravityScale(1.0);
                MovementMode::Grounded
            }
            MovementMode::Grounded => {
                *gravity = GravityScale(0.0);
                MovementMode::Flying
            }
        };

        println!("Switched to mode: {:?}", mode);
    }
}

pub fn is_grounded_mode(query: Query<&MovementMode, With<CameraController>>) -> bool {
    if let Ok(mode) = query.single() {
        *mode == MovementMode::Grounded
    } else {
        false
    }
}

pub fn is_flying_mode(query: Query<&MovementMode, With<CameraController>>) -> bool {
    if let Ok(mode) = query.single() {
        *mode == MovementMode::Flying
    } else {
        false
    }
}
