use crate::{
    LoadState,
    road::{RoadConstructor, systems::*, types::BuildRoadMessage},
    states,
};
use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BuildRoadMessage>();
        app.insert_resource(RoadConstructor::empty());
        app.add_systems(OnEnter(LoadState::Ready), create_road_assets);
        app.add_systems(
            Update,
            spawn_road
                .run_if(in_state(states::BuildSelection::Road).and(in_state(LoadState::Ready))),
        );
    }
}
