use crate::globals::*;
use crate::road::RoadConstructor;
use crate::road::road_construction_functions::{
    build_road_scene, build_road_scene_from_path, create_candidate_road,
};
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;

#[derive(Component, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Road {
    start: GridPos,
    waypoints: Box<[GridPos]>,
    end: GridPos,
    pub start_entity: Entity,
    pub end_entity: Entity,
    pub path: Vec<GridPos>, // Store the exact path here
}

impl Road {
    pub fn from_road_construct(
        construct: &RoadConstructor,
        buildability_map: &BuildabilityMap,
    ) -> Option<Road> {
        let list = construct.get_list();
        let path = create_candidate_road(&list, *buildability_map)?;
        Some(Road {
            start: (*construct.get_start().0)?,
            end: (*construct.get_end().0)?,
            waypoints: construct.get_waypoints().clone().into_boxed_slice(),
            start_entity: (*construct.get_start().1)?,
            end_entity: (*construct.get_end().1)?,
            path,
        })
    }

    pub fn spawn_roads(
        &self,
        commands: &mut Commands,
        buildability_map: &mut BuildabilityMap,
    ) -> Result<(), &'static str> {
        let res = build_road_scene_from_path(&self.path, None);

        let mut children = Vec::with_capacity(res.len());
        for (pos, scene) in res {
            children.push(commands.spawn_scene(scene).id());
            buildability_map.set_real(pos, true);
        }

        commands
            .spawn((self.clone(), Transform::IDENTITY))
            .add_children(&children);

        Ok(())
    }

    pub fn despawn_roads(
        &self,
        commands: &mut Commands,
        buildability_map: &mut BuildabilityMap,
        road_entity: Entity,
    ) {
        // Clear buildability map using the exact stored path
        for pos in &self.path {
            buildability_map.set_real(*pos, false);
        }
        commands.entity(road_entity).despawn();
    }
}
