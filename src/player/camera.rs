use bevy::{
    input::{mouse::MouseMotion},
    prelude::*,
    render::camera::PerspectiveProjection,
};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct MyCameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(),
            near: 0.1,   // set near clipping plane
            far: 1000.0, // set far clipping plane to something reasonable
            ..default()
        }),
        Transform::from_xyz(10.0, 50.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        //RigidBody::Dynamic,
        Collider::capsule_y(0.9, 0.4), // height, radius
        Velocity::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        MyCameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
    ));
}

pub fn camera_mouse_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut MyCameraController, &mut Transform)>,
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