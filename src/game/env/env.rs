use bevy::prelude::*;
use bevy::pbr::DirectionalLightShadowMap;
use super::{assets, terrain};
pub struct EnvPlugin;

impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(DirectionalLightShadowMap { size: 2048 })
            .init_resource::<assets::LogTimer>()
            .init_resource::<assets::LoadingTracker>()
            .add_systems(Startup, assets::declare_assets)
            .add_systems(PostStartup, terrain::spawn_ground_panel)
            .add_systems(PostStartup, terrain::spawn_cube)
            .add_systems(PostStartup, terrain::spawn_directional_light)
            .add_systems(Update, (
                //assets::spawn_loaded_assets,
                assets::log_loading_tracker_state,
                terrain::log_cube_positions_system,
                terrain::animate_light_direction,
            ));
    }
}