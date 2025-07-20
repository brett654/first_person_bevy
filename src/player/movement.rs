use bevy::prelude::*;
use super::camera::MyCameraMarker;
use bevy_rapier3d::prelude::*;

pub fn camera_movement_flying(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MyCameraMarker>>,
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
    //time: Res<Time>,
    mut query: Query<(&mut Velocity, &Transform), With<MyCameraMarker>>,
) {
    let Ok((mut velocity, transform)) = query.single_mut() else { return };

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
    /*
    // Optional vertical movement (flying — disable for walking-only)
    if keys.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }
    */
    if keys.just_pressed(KeyCode::Space) {
        velocity.linvel.y = 6.0; // tweak for jump height
    }

    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
    }

    let speed = 10.0;
    let vertical_velocity = velocity.linvel.y;

    velocity.linvel = direction * speed;
    velocity.linvel.y = vertical_velocity; // preserve gravity fall
}
