use bevy::prelude::*;
use super::{camera::*, movement::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerInput {
                movement: Vec2::ZERO,
                jump: false,
            })
            .add_systems(Startup, (setup_camera))
            .add_systems(Update, camera_mouse_look)
            .add_systems(FixedUpdate, (camera_movement_with_collision));
    }
}