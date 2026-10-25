use crate::{
    factory::Production,
    road::{BuildRoadMessage, RemoveRoadMessage, Road, RoadConstructor},
    states::BuildSelection,
    terrain::BuildabilityMap,
};
use bevy::prelude::*;

pub fn spawn_road(
    mut commands: Commands,
    mut prod: ResMut<Production>,
    mut build_road_message: MessageReader<BuildRoadMessage>,
    mut next_buildselection: ResMut<NextState<BuildSelection>>,
    mut road_constructor: ResMut<RoadConstructor>,
    mut buildability_map: ResMut<BuildabilityMap>,
) {
    for build_road_message in build_road_message.read() {
        match build_road_message {
            BuildRoadMessage::Start(start, factory) => {
                road_constructor.set_start(*start, *factory);
                next_buildselection.set(BuildSelection::Road);
            }
            BuildRoadMessage::WayPoint(pos) => {
                road_constructor.add_waypoints(&[*pos]);
            }
            BuildRoadMessage::End(end, factory) => {
                road_constructor.set_end(*end, *factory);

                let spawned = (|| {
                    let start_entity = (*road_constructor.get_start().1)?;
                    let end_entity = (*road_constructor.get_end().1)?;

                    let start_id = commands.get_entity(start_entity).ok()?.id();
                    let end_id = commands.get_entity(end_entity).ok()?.id();

                    let road = Road::from_road_construct(&road_constructor, &buildability_map)?;
                    road.spawn_roads(&mut commands, &mut buildability_map)
                        .ok()?;

                    prod.add_connection(start_id, end_id);
                    Some(())
                })();

                if spawned.is_none() {
                    println!("Couldn't spawn road");
                }

                next_buildselection.set(BuildSelection::None);
                *road_constructor = RoadConstructor::empty();
            }
        }
    }
}

pub fn remove_road(
    mut commands: Commands,
    mut prod: ResMut<Production>,
    mut remove_road_message: MessageReader<RemoveRoadMessage>,
    mut buildability_map: ResMut<BuildabilityMap>,
    road_query: Query<(Entity, &Road)>,
) {
    for msg in remove_road_message.read() {
        if let Ok((road_entity, road)) = road_query.get(msg.entity) {
            // 1. Remove the connection from the production graph
            prod.remove_connection(road.start_entity, road.end_entity);

            // 2. Clear buildability map and despawn road entities
            road.despawn_roads(&mut commands, &mut buildability_map, road_entity);
        }
    }
}
