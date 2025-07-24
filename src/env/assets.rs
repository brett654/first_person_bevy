use bevy::{
    prelude::*,
    //gltf::GltfAssetLabel,
    scene::SceneRoot,
    //color::palettes::css::*,
};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::geometry::AsyncSceneCollider;

#[derive(Component)] pub struct DragonTag;

#[derive(Component)] pub struct SpiderTag;

#[derive(Component)] pub struct BoxTag;

#[derive(Component)] pub struct SkyboxTag;

#[derive(Component)] pub struct MapTag;

#[derive(Resource)]
pub struct GameAssets {
    /*
    box_scene: Handle<Scene>,
    dragon_scene: Handle<Scene>,
    spider_scene: Handle<Scene>,
    terrain_scene: Handle<Scene>,
    skybox_scene: Handle<Scene>,
    */
    quake_map_scene: Handle<Scene>,
    quake_map: Handle<Gltf>,
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LoadingTracker>()
            .add_systems(Startup, load_assets)
            .add_systems(Update, (
                spawn_loaded_assets,
                //add_colliders_to_scene.after(spawn_loaded_assets),
            ));
    }
}

#[derive(Default, Resource)]
pub struct LoadingTracker {
    spawned: bool,
}

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    /*
    commands.insert_resource(GameAssets {
        box_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("box.glb")),
        dragon_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("fire_dragon.glb")),
        spider_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("black_widow_animated_downloadable.glb")),
        terrain_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("terrain_test.glb")),
        skybox_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("nebula_skybox_16k.glb")),
        quake_map_scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("quake_the_slipgate_complex.glb")),
    });
    */
    let quake_map: Handle<Gltf> = asset_server.load("quake_the_slipgate_complex.glb");
    let quake_map_scene = asset_server.load("quake_the_slipgate_complex.glb#Scene0");

    commands.insert_resource(GameAssets {
        quake_map,
        quake_map_scene,
    });
}

fn spawn_scene_with_collider<T: Component>(
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
        RigidBody::Fixed,
        AsyncSceneCollider::default(), // This triggers collider generation from the scene mesh
        tag,
    ));
}

pub fn spawn_loaded_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut tracker: ResMut<LoadingTracker>,
    standard_materials: ResMut<Assets<StandardMaterial>>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    if tracker.spawned {
        return;
    }

    // Check if all assets are fully loaded
    let all_loaded = [
        /*
        &assets.box_scene,
        &assets.dragon_scene,
        &assets.spider_scene,
        &assets.terrain_scene,
        &assets.skybox_scene,
        */
        &assets.quake_map_scene,
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

    
    // Spawn everything
    /*
    spawn_scene(
        &mut commands,
        assets.box_scene.clone(),
        Transform::from_xyz(2.0, 0.0, 2.0),
        BoxTag,
    );
    */
    /* 
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

    spawn_scene(
        &mut commands,
        assets.terrain_scene.clone(),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(10.0)),
        TerrainTag,
    );
    
    spawn_scene(
        &mut commands,
        assets.skybox_scene.clone(),
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(500.0)), // Adjust scale as needed
        SkyboxTag, // You can define this
    );
    */

    spawn_scene_with_collider(&mut commands, assets.quake_map_scene.clone(), Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(5.0)), MapTag);

    modify_lightmap_materials(gltf_assets, assets, standard_materials);
}

fn modify_lightmap_materials(
    gltfs: Res<Assets<Gltf>>,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(gltf) = gltfs.get(&game_assets.quake_map) {
        for (_, material_handle) in &gltf.named_materials {
            if let Some(mat) = materials.get_mut(material_handle) {
                mat.unlit = true;
                mat.emissive = Color::WHITE.into();
                // You can also assign a lightmap texture here if needed
            }
        }
    }
}