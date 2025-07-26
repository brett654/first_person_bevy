use bevy::{
    color::palettes::css::*,
    prelude::*,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver},
};
use super::assets;
use bevy_rapier3d::prelude::*;

#[derive(Component)] pub struct ShapeTag;

#[derive(Component)] pub struct TerrainTag;

pub fn log_cube_positions_system(
    query: Query<&Transform, With<ShapeTag>>,
    time: Res<Time>,
    mut timer: ResMut<assets::LogTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for transform in &query {
            println!("Cube Transform: {:?}", transform);
        }
    }
}

pub fn spawn_ground_panel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,   
) {
    let panel_width = 200.0;
    let panel_height = 200.0;
    let subdivisions = 10;

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(panel_width, panel_height).subdivisions(subdivisions))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: SILVER.into(),
            perceptual_roughness: 1.0,
            //double_sided: true,
            ..default()
        })),
        Transform::IDENTITY,
        Collider::cuboid(panel_width / 2.0, 0.01, panel_height / 2.0),
        RigidBody::Fixed,
        TerrainTag,
    ));
}

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DARK_GREY.into(),
            perceptual_roughness: 0.3,
            metallic: 0.7,
            ..default()
        })),
        Transform::from_xyz(0.0, 10.0, 0.0),
        GlobalTransform::default(),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        ShapeTag,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GOLDENROD.into(),
            metallic: 1.0,
            ..default()
        })),
        Transform::from_xyz(10.0, 1.0, 0.0),
    ));    
}

pub fn spawn_directional_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 30.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            maximum_distance: 100.0,
            ..default()
        }
        .build(),
    ));
}

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        // Rotate around Y axis slowly
        transform.rotate_y(0.2 * time.delta_secs());
    }
}