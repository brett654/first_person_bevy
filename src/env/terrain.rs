use bevy::{
    color::palettes::css::*,
    prelude::*,
    
    pbr::CascadeShadowConfigBuilder,
    log::{info, warn, error, debug},
    render::{
        mesh::{Indices},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology},
    },
};

use bevy_rapier3d::{prelude::*};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AmbientLight {
            affects_lightmapped_meshes: true,
            color: Color::WHITE,
            brightness: 0.3, // increase if needed
        })
        .add_systems(Startup, (spawn_ground_panel, spawn_cube, spawn_directional_light))
        .add_systems(Update, log_positions_system);
    }
}

#[derive(Component)] pub struct ShapeTag;

#[derive(Component)] pub struct TerrainTag;

fn log_positions_system(query: Query<(&Transform, Entity)>) {
    for (transform, entity) in query.iter() {
        println!("Entity {:?} position: {:?}", entity, transform.translation);
    }
}

fn spawn_ground_panel(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,   
) {
    let width = 100.0;
    let height = 100.0;
 
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(width, height))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::IDENTITY,
        GlobalTransform::default(), // <--- IMPORTANT
        Collider::cuboid(width / 2.0, 0.01, height / 2.0),
        RigidBody::Fixed,
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
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
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
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
        Transform::from_xyz(50.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::X),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::VISIBLE,
        ViewVisibility::default(),
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 100.0,
            maximum_distance: 500.0,
            ..default()
        }
        .build(),
    ));
}

//TODO apply velocities to dynamic objects manually
/*
fn sync_physics_to_transform_system(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&RigidBody, &mut Transform)>,
) {
    for (rigid_body, mut transform) in query.iter_mut() {
        if let Some(position) = rapier_context.get_rigid_body_position(rigid_body) {
            let pos = position.position.translation;
            let rot = position.position.rotation;

            transform.translation = Vec3::new(pos.x, pos.y, pos.z);
            transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
        }
    }
}
*/
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