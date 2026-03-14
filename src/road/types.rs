use std::collections::HashMap;

use crate::terrain::BuildabilityMap;
use bevy::{prelude::*, reflect::List};

#[derive(Resource, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoadConstructor {
    start: Option<IVec2>,
    waypoints: Vec<IVec2>,
    end: Option<IVec2>,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRoadMessage {
    Start(IVec2),
    End(IVec2),
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct RoadAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Road {
    start: IVec2,
    waypoints: Box<[IVec2]>,
    end: IVec2,
}

impl Road {
    pub fn new(points: Vec<IVec2>) -> Result<Road> {
        if points.len() < 2 {
            error!("Too few points");
        }

        let start = points[0];
        let end = points[points.len() - 1];
        if let Some(intermediates) = points.as_slice().get(1..points.len() - 1) {
            Ok(Road {
                start,
                waypoints: Vec::from(intermediates).into_boxed_slice(),
                end,
            })
        } else {
            Ok(Road {
                start,
                waypoints: Box::new([]),
                end,
            })
        }
    }

    pub fn real_pos(pos: &IVec2) -> Vec3 {
        Vec3::new(pos.x as f32, 0., pos.y as f32)
    }

    pub fn create_candidate_road(
        start: &IVec2,
        end: &IVec2,
        filter_map: BuildabilityMap,
    ) -> Option<Vec<IVec2>> {
        if filter_map.get(end.x, end.y) {
            return None;
        }

        let h = |x: IVec2| ((end.x - x.x).abs() + (end.y - x.y).abs());
        let mut next_to_search: Vec<IVec2> = vec![*start];
        let mut f_scores: HashMap<IVec2, i32> = HashMap::new();

        let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();

        let mut g_scores: HashMap<IVec2, i32> = HashMap::new();
        g_scores.insert(*start, 0);

        while !next_to_search.is_empty() {
            let current = next_to_search.remove(0);
            if current == *end {
                let mut total_path = vec![current];
                let mut backward = current;
                while came_from.contains_key(&backward) {
                    backward = came_from[&backward];
                    total_path.push(backward);
                }
                return Some(total_path);
            }

            let neighbours: Vec<IVec2> = [
                current + IVec2::X,
                current + IVec2::Y,
                current - IVec2::X,
                current - IVec2::Y,
            ]
            .iter()
            .filter_map(|neighbour| {
                if neighbour.x < 0 || neighbour.y < 0 || filter_map.get(neighbour.x, neighbour.y) {
                    None
                } else {
                    Some(*neighbour)
                }
            })
            .collect();

            for neighbour in neighbours {
                let this_g_score = g_scores[&current] + 1;
                let g_score = g_scores.get(&neighbour).unwrap_or(&i32::MAX);
                let already_exists = g_scores.contains_key(&neighbour);

                if this_g_score < *g_score {
                    g_scores.insert(neighbour, this_g_score);
                    came_from.insert(neighbour, current);
                    f_scores.insert(neighbour, this_g_score + h(neighbour));
                    if !already_exists {
                        next_to_search.push(neighbour);
                    }
                }
            }
        }

        None
    }

    pub fn spawn_road_segments(
        &self,
        commands: &mut Commands,
        mesh: &Handle<Mesh>,
        material: &Handle<StandardMaterial>,
        buildability_map: &mut BuildabilityMap,
    ) {
        if let Some(path) = Road::create_candidate_road(&self.start, &self.end, *buildability_map) {
            for pos in path {
                let real_pos = Road::real_pos(&pos);

                let res = buildability_map.set_real(pos, true);

                if res.is_err() {
                    println!("Couldn't set build map");
                }

                commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(real_pos.x, real_pos.y, real_pos.z),
                ));
            }
        }
    }
}

impl RoadConstructor {
    pub fn new(start: IVec2, waypoints: Vec<IVec2>, end: IVec2) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints,
            end: Some(end),
        }
    }

    pub fn empty() -> Self {
        RoadConstructor {
            start: None,
            waypoints: vec![],
            end: None,
        }
    }

    pub fn start_new(start: IVec2) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints: vec![],
            end: None,
        }
    }

    pub fn add_waypoints(&mut self, points: &mut dyn Iterator<Item = IVec2>) {
        points.for_each(|point| self.waypoints.push(point));
    }

    pub fn get_start(&self) -> &Option<IVec2> {
        &self.start
    }
    pub fn get_end(&self) -> &Option<IVec2> {
        &self.end
    }
    pub fn get_waypoints(&self) -> &Vec<IVec2> {
        &self.waypoints
    }
    pub fn set_start(&mut self, start: IVec2) {
        self.start = Some(start);
    }
    pub fn set_end(&mut self, end: IVec2) {
        self.end = Some(end);
    }

    pub fn finished(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    pub fn get_list(&self) -> Vec<IVec2> {
        let mut result = self.waypoints.clone();

        if let Some(end) = self.end {
            result.push(end);
        }

        if let Some(start) = self.start {
            result.insert(0, start);
        }

        result
    }
}
