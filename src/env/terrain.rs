use bevy::{
    color::palettes::css::*,
    prelude::*,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
};
use bevy_rapier3d::prelude::*;
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (spawn_ground_panel, spawn_cube, spawn_directional_light));
    }
}

#[derive(Component)] pub struct ShapeTag;

#[derive(Component)] pub struct TerrainTag;

fn spawn_ground_panel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,   
) {
    let width = 200.0;
    let height = 200.0;
 
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::IDENTITY,
        GlobalTransform::default(), // <--- IMPORTANT
        Collider::cuboid(width / 2.0, 0.01, height / 2.0),
        RigidBody::Fixed,
        TerrainTag,
    ));
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DEEP_PINK.into(),
            ..default()
        })),
        Transform::from_xyz(20.0, 10.0, 0.0),
        GlobalTransform::default(), // <--- IMPORTANT
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        ShapeTag,
    ));   
}

fn spawn_directional_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        CascadeShadowConfigBuilder {
            num_cascades: 3,
            first_cascade_far_bound: 100.0,
            maximum_distance: 500.0,
            ..default()
        }
        .build(),
    ));
}