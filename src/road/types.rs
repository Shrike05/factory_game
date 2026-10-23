use crate::globals::*;
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

const SEARCH_DEPTH: u32 = 100;

#[derive(Resource, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoadConstructor {
    start: Option<GridPos>,
    waypoints: Vec<GridPos>,
    end: Option<GridPos>,
    start_entity: Option<Entity>,
    end_entity: Option<Entity>,
}

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildRoadMessage {
    Start(GridPos, Entity),
    WayPoint(GridPos),
    End(GridPos, Entity),
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

    fn find_path_segment(
        start: IVec2,
        end: IVec2,
        filter_map: &BuildabilityMap,
        global_visited: &HashSet<GridPos>,
    ) -> Option<Vec<GridPos>> {
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
                let mut segment = vec![GridPos::new(current.x as u32, current.y as u32)];
                let mut backward = current;
                while came_from.contains_key(&backward) {
                    backward = came_from[&backward];
                    segment.push(GridPos::new(backward.x as u32, backward.y as u32));
                }
                segment.reverse();
                return Some(segment);
            }

            let neighbours: Vec<IVec2> = [
                current + IVec2::X,
                current + IVec2::Y,
                current - IVec2::X,
                current - IVec2::Y,
            ]
            .iter()
            .filter_map(|neighbour| {
                let grid_pos = GridPos::new(neighbour.x as u32, neighbour.y as u32);
                if neighbour.x < 0
                    || neighbour.y < 0
                    || filter_map.get(neighbour.x as u32, neighbour.y as u32)
                    || global_visited.contains(&grid_pos)
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

    /// Creates a candidate road by routing through start, all waypoints, and end in order,
    /// ensuring no self-intersections occur across segments.
    pub fn create_candidate_road(&self, filter_map: BuildabilityMap) -> Option<Vec<GridPos>> {
        let mut points = Vec::with_capacity(2 + self.waypoints.len());
        points.push(self.start);
        points.extend(self.waypoints.iter().cloned());
        points.push(self.end);

        let mut full_path: Vec<GridPos> = Vec::new();
        let mut global_visited: HashSet<GridPos> = HashSet::new();

        // Chain A* pathfinding between each consecutive pair of points
        for window in points.windows(2) {
            let p_start = IVec2::new(window[0].x as i32, window[0].y as i32);
            let p_end = IVec2::new(window[1].x as i32, window[1].y as i32);

            let segment = Self::find_path_segment(p_start, p_end, &filter_map, &global_visited)?;

            if full_path.is_empty() {
                global_visited.extend(segment.iter().cloned());
                full_path.extend(segment);
            } else {
                // Skip the first element to avoid duplicating the joint node,
                // but mark the rest as visited and add them to the path
                let slice = &segment[1..];
                global_visited.extend(slice.iter().cloned());
                full_path.extend(slice.iter().cloned());
            }
        }

        // Strip the absolute start and end to return only the inner waypoints
        if full_path.len() <= 2 {
            Some(vec![])
        } else {
            Some(full_path[1..full_path.len() - 1].to_vec())
        }
    }

    pub fn spawn_road_segments(
        &self,
        commands: &mut Commands,
        mesh: &Handle<Mesh>,
        material: &Handle<StandardMaterial>,
        buildability_map: &mut BuildabilityMap,
    ) -> Option<()> {
        if let Some(path) = self.create_candidate_road(*buildability_map) {
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

            Some(())
        } else {
            None
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
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn empty() -> Self {
        RoadConstructor {
            start: None,
            waypoints: vec![],
            end: None,
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn start_new(start: GridPos) -> Self {
        RoadConstructor {
            start: Some(start),
            waypoints: vec![],
            end: None,
            start_entity: None,
            end_entity: None,
        }
    }

    pub fn add_waypoints(&mut self, points: &Vec<GridPos>) {
        points.iter().for_each(|point| self.waypoints.push(*point));
    }

    pub fn get_start(&self) -> (&Option<GridPos>, &Option<Entity>) {
        (&self.start, &self.start_entity)
    }
    pub fn get_end(&self) -> (&Option<GridPos>, &Option<Entity>) {
        (&self.end, &self.end_entity)
    }
    pub fn get_waypoints(&self) -> &Vec<GridPos> {
        &self.waypoints
    }
    pub fn set_start(&mut self, start: GridPos, entity: Entity) {
        self.start = Some(start);
        self.start_entity = Some(entity)
    }
    pub fn set_end(&mut self, end: GridPos, entity: Entity) {
        self.end = Some(end);
        self.end_entity = Some(entity)
    }

    pub fn set_end_coordinate(&mut self, end: GridPos) {
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
