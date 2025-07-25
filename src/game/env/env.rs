use bevy::prelude::*;

use super::{assets, terrain};
pub struct EnvPlugin;

impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<assets::LoadingTracker>()
            .add_systems(Startup, assets::load_assets)
            .add_systems(PostStartup, (
                //terrain::spawn_ground_panel,
                //terrain::spawn_cube,
                terrain::spawn_directional_light
            ))
            .add_systems(Update, (
                assets::spawn_loaded_assets,
                //add_colliders_to_scene.after(spawn_loaded_assets),
            ));
    }
}