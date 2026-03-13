use crate::preview::systems::{init_preview, preview_factory, preview_road};
use bevy::prelude::*;

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_preview);
        app.add_systems(Update, (preview_factory, preview_road));
    }
}
