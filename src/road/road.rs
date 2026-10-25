use crate::globals::*;
use crate::road::RoadConstructor;
use crate::road::road_construction_functions::{build_road_scene, create_candidate_road};
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;

#[derive(Component, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Road {
    start: GridPos,
    waypoints: Box<[GridPos]>,
    end: GridPos,
    pub start_entity: Entity,
    pub end_entity: Entity,
}

impl Road {
    pub fn from_road_construct(construct: &RoadConstructor) -> Option<Road> {
        Some(Road {
            start: (*construct.get_start().0)?,
            end: (*construct.get_end().0)?,
            waypoints: construct.get_waypoints().clone().into_boxed_slice(),
            start_entity: (*construct.get_start().1)?,
            end_entity: (*construct.get_end().1)?,
        })
    }

    pub fn real_pos(pos: &GridPos) -> Vec3 {
        Vec3::new(pos.x as f32, 0., pos.y as f32)
    }

    pub fn get_list(&self) -> Vec<GridPos> {
        let mut result = self.waypoints.to_vec();
        result.push(self.end);
        result.insert(0, self.start);
        result
    }

    pub fn create_candidate_road(&self, filter_map: BuildabilityMap) -> Option<Vec<GridPos>> {
        let list = self.get_list();
        create_candidate_road(&list, filter_map)
    }

    pub fn spawn_roads(&self, commands: &mut Commands, buildability_map: &mut BuildabilityMap) {
        let res = self.build_road_scenes(buildability_map, None).unwrap();

        let mut children = vec![];
        res.into_iter().for_each(|(pos, scene)| {
            children.push(commands.spawn_scene(scene).id());
            buildability_map.set_real(pos, true);
        });

        commands
            .spawn((self.clone(), Transform::IDENTITY))
            .add_children(&children);
    }

    pub fn build_road_scenes(
        &self,
        buildability_map: &BuildabilityMap,
        road_color: Option<Color>,
    ) -> Option<Vec<(UVec2, Box<dyn Scene>)>> {
        let list = self.get_list();
        build_road_scene(&list, buildability_map, road_color)
    }
}
