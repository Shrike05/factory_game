use bevy::prelude::*;

pub struct CameraPlugin;
use crate::camera::setup::{setup, setup_light};
use crate::camera::systems::move_camera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, setup_light));
        app.add_systems(Update, move_camera);
    }
}
