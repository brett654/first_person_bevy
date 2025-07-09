use bevy::{
    prelude::*,
    pbr::CascadeShadowConfigBuilder,
};

pub fn setup_light(mut commands: Commands) {     
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        CascadeShadowConfigBuilder::default().build(),
    ));
    
}