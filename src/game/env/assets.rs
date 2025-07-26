use bevy::{
    prelude::*,
    //gltf::GltfAssetLabel,
    scene::SceneRoot,
    //color::palettes::css::*,
};
use bevy_rapier3d::geometry::AsyncSceneCollider;

#[derive(Component)] pub struct MapTag;

#[derive(Resource)]
pub struct GameAssets {
    quake_map_scene: Handle<Scene>,
    quake_map: Handle<Gltf>,
}

#[derive(Default, Resource)]
pub struct LoadingTracker {
    spawned: bool,
}

#[derive(Resource)]
pub struct LogTimer(pub Timer);
impl Default for LogTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

pub fn log_loading_tracker_state(
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
    tracker: Res<LoadingTracker>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        info!("LoadingTracker state: spawned = {}", tracker.spawned);
    }
}

pub fn declare_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    spawn_scene_with_collider(&mut commands, assets.quake_map_scene.clone(), Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(5.0)), MapTag);
    modify_lightmap_materials(gltf_assets, assets, standard_materials);
}

pub fn modify_lightmap_materials(
    gltfs: Res<Assets<Gltf>>,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(gltf) = gltfs.get(&game_assets.quake_map) {
        for (_, material_handle) in &gltf.named_materials {
            if let Some(mat) = materials.get_mut(material_handle) {
                mat.unlit = true;
                mat.emissive = Color::WHITE.into();
            }
        }
    }
}