use crate::{preview::road_preview::*, states};
use bevy::prelude::*;
use states::BuildSelection;

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, preview_road.run_if(in_state(BuildSelection::Road)));
        app.add_systems(OnExit(BuildSelection::Road), stop_build_road);
    }
}
