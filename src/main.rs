mod cursor;
use cursor::cursor::CursorPlugin;

mod player;
use player::player::PlayerPlugin;

mod env;
use env::{AssetPlugin, LightPlugin, TerrainPlugin};

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.678, 0.847, 0.902))) // light sky blue
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            CursorPlugin,
            PlayerPlugin,
            LightPlugin,
            TerrainPlugin,
            AssetPlugin,
        ))
        .run();
}
