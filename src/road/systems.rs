use crate::{
    factory::Production,
    road::{BuildRoadMessage, Road, RoadConstructor},
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
                if let Ok(road) = Road::new(road_constructor.get_list()) {
                    let result = road.spawn_roads(&mut commands, &mut buildability_map);

                    let start_entity = commands
                        .get_entity(road_constructor.get_start().1.unwrap())
                        .unwrap()
                        .id();
                    let end_entity = commands
                        .get_entity(road_constructor.get_end().1.unwrap())
                        .unwrap()
                        .id();

                    prod.add_connection(start_entity, end_entity);
                } else {
                    println!("Couldn't spawn road");
                }

                next_buildselection.set(BuildSelection::None);
                *road_constructor = RoadConstructor::empty();
            }
        }
    }
}
