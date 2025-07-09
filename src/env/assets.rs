use bevy::{prelude::*,
    gltf::GltfAssetLabel,
    scene::SceneRoot,
};

pub fn load_gltf(
    mut commands: Commands,
    asset_server : Res<AssetServer>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("box.glb"),
    )));
}