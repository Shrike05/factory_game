use crate::{
    LoadState,
    preview::{factory_preview::*, road_preview::*},
    states,
};
use bevy::prelude::*;
use states::{BuildSelection, InFactoryMode};

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadState::Ready), init_factory_preview);
        app.add_systems(
            Update,
            preview_road.run_if(in_state(BuildSelection::Road).and(in_state(LoadState::Ready))),
        );
        app.add_systems(OnExit(BuildSelection::Road), stop_build_road);
        app.add_computed_state::<InFactoryMode>()
            .add_systems(
                Update,
                preview_factory
                    .run_if(in_state(InFactoryMode::True).and(in_state(LoadState::Ready))),
            )
            .add_systems(OnExit(InFactoryMode::True), stop_preview_factory);
    }
}
