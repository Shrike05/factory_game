use crate::globals::*;
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;
use std::collections::HashMap;

const SEARCH_DEPTH: u32 = 100;

#[derive(Resource, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoadConstructor {
    start: Option<GridPos>,
    waypoints: Vec<GridPos>,
    end: Option<GridPos>,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRoadMessage {
    Start(GridPos),
    End(GridPos),
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct RoadAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Road {
    start: GridPos,
    waypoints: Box<[GridPos]>,
    end: GridPos,
}

impl Road {
    pub fn new(points: Vec<GridPos>) -> Result<Road> {
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

    pub fn real_pos(pos: &GridPos) -> Vec3 {
        Vec3::new(pos.x as f32, 0., pos.y as f32)
    }

    pub fn create_candidate_road(
        start_real: &GridPos,
        end_real: &GridPos,
        filter_map: BuildabilityMap,
    ) -> Option<Vec<GridPos>> {
        let start = IVec2::new(start_real.x as i32, start_real.y as i32);
        let end = IVec2::new(end_real.x as i32, end_real.y as i32);

        if filter_map.get(end_real.x, end_real.y) {
            return None;
        }

        let h = |x: IVec2| (end.x - x.x).abs() + (end.y - x.y).abs();
        let mut next_to_search: Vec<IVec2> = vec![start];
        let mut f_scores: HashMap<IVec2, i32> = HashMap::new();

        let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();

        let mut g_scores: HashMap<IVec2, i32> = HashMap::new();
        g_scores.insert(start, 0);

        let mut current_depth = 0_u32;
        while !next_to_search.is_empty() && current_depth < SEARCH_DEPTH {
            current_depth += 1;

            let current = next_to_search.remove(0);
            if current == end {
                let mut total_path = vec![GridPos::new(current.x as u32, current.y as u32)];
                let mut backward = current;
                while came_from.contains_key(&backward) {
                    backward = came_from[&backward];
                    let backward_gridpos = UVec2::new(backward.x as u32, backward.y as u32);
                    total_path.push(backward_gridpos);
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
                if neighbour.x < 0
                    || neighbour.y < 0
                    || filter_map.get(neighbour.x as u32, neighbour.y as u32)
                {
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

#[allow(unused)]
impl RoadConstructor {
    pub fn new(start: GridPos, waypoints: Vec<GridPos>, end: GridPos) -> Self {
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

    pub fn start_new(start: GridPos) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints: vec![],
            end: None,
        }
    }

    pub fn add_waypoints(&mut self, points: &mut dyn Iterator<Item = GridPos>) {
        points.for_each(|point| self.waypoints.push(point));
    }

    pub fn get_start(&self) -> &Option<GridPos> {
        &self.start
    }
    pub fn get_end(&self) -> &Option<GridPos> {
        &self.end
    }
    pub fn get_waypoints(&self) -> &Vec<GridPos> {
        &self.waypoints
    }
    pub fn set_start(&mut self, start: GridPos) {
        self.start = Some(start);
    }
    pub fn set_end(&mut self, end: GridPos) {
        self.end = Some(end);
    }

    pub fn finished(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    pub fn get_list(&self) -> Vec<GridPos> {
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
