use crate::{preview::factory_preview::*, preview::road_preview::*, states};
use bevy::prelude::*;
use states::{BuildSelection, InFactoryMode};

pub struct PreviewPlugin;

impl Plugin for PreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_factory_preview);
        app.add_systems(Update, preview_road.run_if(in_state(BuildSelection::Road)));
        app.add_systems(OnExit(BuildSelection::Road), stop_build_road);
        app.add_computed_state::<InFactoryMode>()
            .add_systems(
                Update,
                preview_factory.run_if(in_state(InFactoryMode::True)),
            )
            .add_systems(OnExit(InFactoryMode::True), stop_preview_factory);
    }
}
