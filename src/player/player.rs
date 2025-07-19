use bevy::prelude::*;
use super::{camera::*, movement::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_mouse_look, camera_movement));
    }
}