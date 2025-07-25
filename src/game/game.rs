use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::{player::player, env::env, cursor::cursor};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            cursor::CursorPlugin,
            player::PlayerPlugin,
            env::EnvPlugin,
        ));
    }
}