use crate::{
    factory::Production,
    road::{
        BuildRoadMessage, RoadConstructor,
        types::{Road, RoadAssets},
    },
    states::BuildSelection,
    terrain::BuildabilityMap,
};
use bevy::prelude::*;

pub fn create_road_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(RoadAssets {
        mesh: meshes.add(Cuboid::new(0.9, 0.3, 0.9)),
        material: materials.add(Color::srgb(0.51, 0.19, 0.)),
    });
}

pub fn spawn_road(
    mut commands: Commands,
    mut prod: ResMut<Production>,
    road_assets: Res<RoadAssets>,
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
                road_constructor.add_waypoints(&vec![*pos]);
            }
            BuildRoadMessage::End(end, factory) => {
                road_constructor.set_end(*end, *factory);
                if let Ok(road) = Road::new(road_constructor.get_list()) {
                    road.spawn_road_segments(
                        &mut commands,
                        &road_assets.mesh,
                        &road_assets.material,
                        &mut buildability_map,
                    );

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
