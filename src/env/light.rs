use bevy::{
    prelude::*,
    color::palettes::css::*,
    pbr::CascadeShadowConfigBuilder,
};
use std::f32::consts::PI;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_light);
    }
}

pub fn setup_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AmbientLight {
        color: ORANGE_RED.into(),
        brightness: 200.0,
        ..default()
    });

    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: RED.into(),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 2.0, 0.0),
        children![(
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                emissive: LinearRgba::new(5.0, 15.0, 0.0, 0.0),
                ..default()
            })),
        )],
    ));
     
    // green spot light
    commands.spawn((
        SpotLight {
            intensity: 100_000.0,
            color: LIME.into(),
            shadows_enabled: true,
            inner_angle: 0.6,
            outer_angle: 0.8,
            ..default()
        },
        Transform::from_xyz(-1.0, 2.0, 0.0).looking_at(Vec3::new(-1.0, 0.0, 0.0), Vec3::Z),
        children![(
            Mesh3d(meshes.add(Capsule3d::new(0.1, 0.125))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: LIME.into(),
                emissive: LinearRgba::new(0.0, 4.0, 0.0, 0.0),
                ..default()
            })),
            Transform::from_rotation(Quat::from_rotation_x(PI / 2.0)),
        )],
    ));

    // blue point light
    commands.spawn((
        PointLight {
            intensity: 100_000.0,
            color: BLUE.into(),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 4.0, 0.0),
        children![(
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(32, 18))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: BLUE.into(),
                emissive: LinearRgba::new(0.0, 0.0, 713.0, 0.0),
                ..default()
            })),
        )],
    ));

    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));
    
}