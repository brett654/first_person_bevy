mod cursor;
use cursor::cursor::CursorPlugin;

mod player;
use player::player::PlayerPlugin;

mod env;
use env::{AssetPlugin, LightPlugin};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorPlugin, AssetPlugin, PlayerPlugin, LightPlugin))
        .run();
}
