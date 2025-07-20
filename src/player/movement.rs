use bevy::prelude::*;
use super::camera::{MyCameraController};
use bevy_rapier3d::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub jump: bool,
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub speed: f32,
    pub gravity: f32,
    pub jump_force: f32,
    pub grounded: bool,
}

pub fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player {
            velocity: Vec3::ZERO,
            speed: 5.0,
            gravity: 9.81,
            jump_force: 5.0,
            grounded: false,
        },
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.9, 0.4),
        KinematicCharacterController::default(),
    ));
}

pub fn update_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    input.movement = Vec2::ZERO;
    input.jump = keys.just_pressed(KeyCode::Space);

    if keys.pressed(KeyCode::KeyW) {
        input.movement.y += 1.;
    }
    if keys.pressed(KeyCode::KeyS) {
        input.movement.y -= 1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.movement.x += 1.;
    }
    if keys.pressed(KeyCode::KeyA) {
        input.movement.x -= 1.;
    }
}


pub fn update_movement(
    time: Res<Time<Fixed>>,
    input: Res<PlayerInput>,
    camera_query: Query<&Transform, With<Camera>>,
    mut query: Query<(&mut Player, &mut KinematicCharacterController, Option<&KinematicCharacterControllerOutput>)>,
) {
    let Ok(camera_transform) = camera_query.single() else { return };

    for (mut player, mut controller, output) in &mut query {
        // Update grounded from physics controller output
        if let Some(out) = output {
            player.grounded = out.grounded;
            if player.grounded && player.velocity.y < 0.0 {
                player.velocity.y = 0.0; // reset downward velocity when grounded
            }
        }

        let forward = camera_transform.forward().xz().normalize_or_zero();
        let right = camera_transform.right().xz().normalize_or_zero();

        let input_dir = forward * input.movement.y + right * input.movement.x;

        // Instead of adding to velocity, directly set horizontal velocity to input direction * speed
        if let Some(dir) = input_dir.try_normalize() {
            player.velocity.x = dir.x * player.speed;
            player.velocity.z = dir.y * player.speed;
        } else {
            // No input - slow horizontal movement gradually (friction)
            player.velocity.x *= 0.8;
            player.velocity.z *= 0.8;
        }

        if input.jump && player.grounded {
            player.velocity.y = player.jump_force;
            player.grounded = false;
        }

        // Gravity
        player.velocity.y -= player.gravity * time.timestep().as_secs_f32();

        controller.translation = Some(player.velocity * time.timestep().as_secs_f32());
    }
}

pub fn camera_movement_flying(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MyCameraController>>,
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
    mut query: Query<(&mut Velocity, &Transform), With<MyCameraController>>,
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
