use bevy::{
    prelude::*,
    pbr::CascadeShadowConfigBuilder,
};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_light);
    }
}

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