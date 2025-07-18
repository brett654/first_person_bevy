use bevy::{
    input::{mouse::MouseMotion},
    prelude::*
};

#[derive(Component)]
pub struct MyCameraMarker {
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
        MyCameraMarker {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
        Transform::from_xyz(10.0, 50.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}

pub fn camera_mouse_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut MyCameraMarker, &mut Transform)>,
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