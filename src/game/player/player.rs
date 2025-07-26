use bevy::prelude::*;
use super::{camera_controller::*, player_movement::*};

use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, (
                camera_mouse_look,
                switch_movement_mode,
                camera_movement_flying.run_if(is_flying_mode)
            ))
            .add_systems(FixedUpdate, camera_movement_with_collision.run_if(is_grounded_mode));
    }
}

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

#[derive(Component,PartialEq, Debug)]
pub enum MovementMode {
    Grounded,
    Flying,
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_height = 1.5;
    let player_radius = 0.4;
    let spawn_spot = Vec3::new(70.0,20.0,-20.0);

    let player_entity = commands.spawn((
        Transform::from_translation(spawn_spot)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(player_height, player_radius),
        Velocity::default(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        CameraController {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
        MovementMode::Grounded,
        Player {},
    )).id();

    let camera_entity = commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            aspect_ratio: 16.0 / 9.0,
            fov: std::f32::consts::FRAC_PI_3,
            near: 0.1,
            far: 1000.0,
            ..default()
        }),
        Camera {
            hdr : true,
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("enviorement_map/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("enviorement_map/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 2000.0,
            ..default()
        },
    )).id();

    commands.entity(player_entity).add_child(camera_entity);
}
