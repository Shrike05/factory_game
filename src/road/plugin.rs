use crate::road::{RemoveRoadMessage, RoadConstructor, systems::*, types::BuildRoadMessage};
use bevy::prelude::*;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BuildRoadMessage>();
        app.add_message::<RemoveRoadMessage>();
        app.insert_resource(RoadConstructor::empty());
        app.add_systems(Update, (spawn_road, remove_road));
    }
}
