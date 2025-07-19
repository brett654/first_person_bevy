mod cursor;
use cursor::cursor::CursorPlugin;

mod player;
use player::{setup_camera, camera_mouse_look, camera_movenent};

mod env;
use env::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorPlugin))
        .add_systems(Startup, (setup_camera, load_assets, setup_light))
        .add_systems(Update, (
            camera_mouse_look,
            camera_movenent,
            spawn_loaded_assets,  // <--- Run this once assets are ready
        ))
        .run();
}
