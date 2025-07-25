use bevy::{
    input::{mouse::MouseMotion},
    prelude::*,
};

use super::player;
/*
pub fn log_camera_positions_system(query: Query<&Transform, With<CameraController>>) {
    for transform in &query {
        println!("Camera Transform: {:?}", transform);
    }
}
*/
pub fn camera_mouse_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut player::CameraController, &mut Transform)>,
    windows: Query<&Window>,
) {
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    if !window.focused {
        return;
    }

    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    for (mut camera, mut transform) in &mut query {
        camera.yaw -= delta.x * camera.sensitivity;
        camera.pitch -= delta.y * camera.sensitivity;

        // Clamp pitch to prevent flipping
        camera.pitch = camera.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);

        // Recalculate transform
        let yaw_rotation = Quat::from_rotation_y(camera.yaw);
        let pitch_rotation = Quat::from_rotation_x(camera.pitch);
        transform.rotation = yaw_rotation * pitch_rotation;
    }
}