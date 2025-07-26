use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::{player::player, env::env, cursor::cursor};
use super::game_state::GameState;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            cursor::CursorPlugin,
            player::PlayerPlugin,
            env::EnvPlugin,
        ));
    }
}