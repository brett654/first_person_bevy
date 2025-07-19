use bevy::{
    prelude::*,
    gltf::GltfAssetLabel,
    scene::SceneRoot,
};

#[derive(Component)] pub struct DragonTag;

#[derive(Component)] pub struct TerrainTag;

#[derive(Component)] pub struct SpiderTag;

#[derive(Component)] pub struct BoxTag;

#[derive(Resource)]
pub struct GameAssets {
    box_scene: Handle<Scene>,
    dragon_scene: Handle<Scene>,
    spider_scene: Handle<Scene>,
    terrain_scene: Handle<Scene>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingTracker>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, spawn_loaded_assets);
    }
}

#[derive(Default, Resource)]
pub struct LoadingTracker {
    spawned: bool,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        box_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("box.glb")),
        dragon_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("fire_dragon.glb")),
        spider_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("black_widow_animated_downloadable.glb")),
        terrain_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("terrain_dristibute_gn.glb")),
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
        //NoFrustumCulling,
        tag,
    ));
}

pub fn spawn_loaded_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut tracker: ResMut<LoadingTracker>,
) {
    if tracker.spawned {
        return;
    }

    // Check if all assets are fully loaded
    let all_loaded = [
        &assets.box_scene,
        &assets.dragon_scene,
        &assets.spider_scene,
        &assets.terrain_scene,
    ]
    .iter()
    .all(|handle| {
        matches!(
            asset_server.get_load_state(*handle),
            Some(bevy::asset::LoadState::Loaded)
        )
    });

    // If not yet ready, return and try again next frame
    if !all_loaded {
        return;
    }

    tracker.spawned = true;

    /*
    // Spawn everything
    spawn_scene(
        &mut commands,
        assets.box_scene.clone(),
        Transform::from_xyz(2.0, 0.0, 2.0),
        BoxTag,
    );

    spawn_scene(
        &mut commands,
        assets.dragon_scene.clone(),
        Transform::from_xyz(5.0, 0.0, -2.0),
        DragonTag,
    );

    spawn_scene(
        &mut commands,
        assets.spider_scene.clone(),
        Transform::from_xyz(-2.0, 0.0, 4.0),
        SpiderTag,
    );
    */
    spawn_scene(
        &mut commands,
        assets.terrain_scene.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(10.0)),
        TerrainTag,
    );
}
