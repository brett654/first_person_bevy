use bevy::{
    prelude::*,
    gltf::GltfAssetLabel,
    scene::{SceneRoot},
};

pub fn load_assets(
    mut commands: Commands,
    asset_server : Res<AssetServer>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("box.glb"),
    )));

    commands.spawn((
        SceneRoot(asset_server.load(
            GltfAssetLabel::Scene(0).from_asset("bathroom_floor.glb"),
        )),
        Transform::from_xyz(0.0, -1.0, 0.0).with_scale(Vec3::splat(1.0)),
        GlobalTransform::default(),
    ));
}
