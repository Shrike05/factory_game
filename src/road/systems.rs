use crate::road::{
    BuildRoadMessage, RoadConstructor,
    types::{Road, RoadAssets},
};
use crate::states::*;
use bevy::prelude::*;
use bevy_terrain::*;

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
    road_assets: Res<RoadAssets>,
    mut build_road_message: MessageReader<BuildRoadMessage>,
    mut road_constructor: ResMut<RoadConstructor>,
    mut buildability_map: ResMut<BuildabilityMap>,
) {
    for _ in build_road_message.read() {
        if road_constructor.finished()
            && let Ok(road) = Road::new(road_constructor.get_list())
        {
            road.spawn_road_segments(
                &mut commands,
                &road_assets.mesh,
                &road_assets.material,
                &mut buildability_map,
            );

            *road_constructor = RoadConstructor::empty();
        }
    }
}

pub fn build_road_event(
    mut ev: MessageReader<TileClickedMessage>,
    mut road_writer: MessageWriter<BuildRoadMessage>,
    mut road_constructor: ResMut<RoadConstructor>,
) {
    for build_ev in ev.read() {
        if road_constructor.get_start().is_some() {
            road_writer.write(BuildRoadMessage::End(*build_ev.get_pos()));
            road_constructor.set_end(*build_ev.get_pos());
        } else {
            road_writer.write(BuildRoadMessage::Start(*build_ev.get_pos()));
            road_constructor.set_start(*build_ev.get_pos());
        }
    }
}
