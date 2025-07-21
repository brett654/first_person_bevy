use bevy::{
    color::palettes::css::*,
    prelude::*,
    
    render::{
        mesh::{Indices},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology},
    },
};
use bevy_rapier3d::prelude::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground_panel, spawn_cube));
    }
}

#[derive(Component)] pub struct ShapeTag;

#[derive(Component)] pub struct TerrainTag;

fn spawn_ground_panel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,   
) {
    let width = 100.0;
    let height = 100.0;
 
    // ground plane
    commands.spawn( (
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::IDENTITY,
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
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: DEEP_PINK.into(),
            ..default()
        })),
        Transform::from_xyz(20.0, 10.0, 0.0),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
    ));   
}

pub fn create_panel_mesh(width: f32, height: f32) -> Mesh {
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    let vertices = vec![
        [-half_w, 0.0, -half_h], // bottom left
        [half_w, 0.0, -half_h],  // bottom right
        [half_w, 0.0, half_h],   // top right
        [-half_w, 0.0, half_h],  // top left
    ];

    let normals = vec![[0.0, 1.0, 0.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices = vec![0, 2, 1, 0, 3, 2];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}