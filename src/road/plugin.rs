use crate::{
    road::{RoadConstructor, systems::*, types::BuildRoadMessage},
    states,
};
use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BuildRoadMessage>();
        app.insert_resource(RoadConstructor::empty());
        app.add_systems(Startup, create_road_assets);
        app.add_systems(
            Update,
            (spawn_road, build_road_event).run_if(in_state(states::BuildSelection::Road)),
        );
    }
}
