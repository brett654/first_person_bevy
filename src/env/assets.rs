use bevy::{
    prelude::*,
    gltf::GltfAssetLabel,
    render::view::NoFrustumCulling,
    scene::SceneRoot,
};

#[derive(Component)]
pub struct DragonTag;

#[derive(Component)]
pub struct TerrainTag;

#[derive(Component)]
pub struct SpiderTag;

#[derive(Component)]
pub struct BoxTag;

#[derive(Resource)]
pub struct GameAssets {
    box_scene: Handle<Scene>,
    dragon_scene: Handle<Scene>,
    spider_scene: Handle<Scene>,
    terrain_scene: Handle<Scene>,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        box_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("box.glb")),
        dragon_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("fire_dragon.glb")),
        spider_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("black_widow_animated_downloadable.glb")),
        terrain_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("swampy_mountain_lake.glb")),
    });
}


fn spawn_scene<T: Component>(
    commands: &mut Commands,
    handle: Handle<Scene>,
    transform: Transform,
    tag: T,
) {
    commands.spawn((
        SceneRoot(handle),
        transform,
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        NoFrustumCulling,
        tag,
    ));
}

pub fn spawn_loaded_assets(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ready: Local<bool>,
) {
    if *ready { return; }
    // check all handles are loaded...
    *ready = true;

    //spawn_scene(&mut commands, assets.spider_scene.clone(), Transform::from_xyz(0.0, 0.0, 0.0), SpiderTag);
    //spawn_scene(&mut commands, assets.dragon_scene.clone(), Transform::from_xyz(0.0, 0.0, 0.0), DragonTag);
    spawn_scene(&mut commands, assets.terrain_scene.clone(), Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(500.0)), TerrainTag);
}
