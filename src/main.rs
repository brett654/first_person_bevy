use bevy::prelude::*;

mod game;
use game::{game::GamePlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.678, 0.847, 0.902)))
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
