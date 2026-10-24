use crate::globals::*;
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

const SEARCH_DEPTH: u32 = 100;

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
            for (pos, path_type) in analyze_path(&path) {
                let world_pos = Road::real_pos(&pos);

                let res = buildability_map.set_real(pos, true);

                if res.is_err() {
                    println!("Couldn't set build map");
                }

                match path_type {
                    PathSegmentType::Start { outgoing } => {
                        // Spawn start cap mesh, rotate based on `outgoing` vector
                        let rotation = direction_to_rotation(outgoing);
                        commands.spawn_scene(bsn! {
                            Transform::from_translation(world_pos).with_rotation(rotation)
                            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
                            @straight_path_mesh()
                        });
                    }
                    PathSegmentType::Straight { direction } => {
                        let rotation = direction_to_rotation(direction);
                        commands.spawn_scene(bsn! {
                            Transform::from_translation(world_pos).with_rotation(rotation)
                            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
                            @straight_path_mesh()
                        });
                    }
                    PathSegmentType::Corner { incoming, outgoing } => {
                        // Corners depend on the exact turn combination (e.g., North-to-East vs South-to-East)
                        let rotation = corner_to_rotation(incoming, outgoing);
                        commands.spawn_scene(bsn! {
                            Transform::from_translation(world_pos).with_rotation(rotation)
                            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
                            @corner_mesh()
                        });
                    }
                    PathSegmentType::End { incoming } => {
                        let rotation = direction_to_rotation(incoming);
                        commands.spawn_scene(bsn! {
                            Transform::from_translation(world_pos).with_rotation(rotation)
                            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
                            @straight_path_mesh()
                        });
                    }
                    PathSegmentType::SingleNode => {
                        // Handle fallback for a path of length 1
                    }
                }
            }

            Some(())
        } else {
            None
        }
    }
}

pub fn straight_path_mesh() -> impl Scene {
    bsn! {
        Mesh3d(asset_value(Cuboid::new(0.5, 0.2, 1.)))
    }
}

pub fn corner_mesh() -> impl Scene {
    bsn! {
        Children [
            // First arm of the L-bend (e.g., stretching along Z)
            Transform::from_xyz(0.0, 0.0, -0.125)
            Mesh3d(asset_value(Cuboid::new(0.5, 0.2, 0.75)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
            --
            // Second arm of the L-bend (e.g., branching out along X)
            Transform::from_xyz(0.125, 0.0, 0.0)
            Mesh3d(asset_value(Cuboid::new(0.75, 0.2, 0.5)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
        ]
    }
}

#[derive(Debug)]
pub enum PathSegmentType {
    Start { outgoing: IVec2 },
    End { incoming: IVec2 },
    Straight { direction: IVec2 },
    Corner { incoming: IVec2, outgoing: IVec2 },
    SingleNode,
}

pub fn analyze_path(path: &[UVec2]) -> Vec<(UVec2, PathSegmentType)> {
    let mut mesh_data = Vec::new();

    if path.is_empty() {
        return mesh_data;
    }

    let len = path.len();

    for (i, &current_uvec) in path.iter().enumerate() {
        let current = current_uvec.as_ivec2();

        // 1. Calculate incoming direction (from previous node)
        let incoming = if i > 0 {
            Some(current - path[i - 1].as_ivec2())
        } else {
            None
        };

        // 2. Calculate outgoing direction (to next node)
        let outgoing = if i < len - 1 {
            Some(path[i + 1].as_ivec2() - current)
        } else {
            None
        };

        // 3. Determine the segment type based on connections
        let segment_type = match (incoming, outgoing) {
            (None, Some(out_dir)) => PathSegmentType::Start { outgoing: out_dir },
            (Some(in_dir), None) => PathSegmentType::End { incoming: in_dir },
            (Some(in_dir), Some(out_dir)) => {
                if in_dir == out_dir {
                    PathSegmentType::Straight { direction: in_dir }
                } else {
                    PathSegmentType::Corner {
                        incoming: in_dir,
                        outgoing: out_dir,
                    }
                }
            }
            (None, None) => PathSegmentType::SingleNode,
        };

        mesh_data.push((current_uvec, segment_type));
    }

    mesh_data
}

// Helper to convert grid directions into 2D rotations (Quaternions)
fn direction_to_rotation(dir: IVec2) -> Quat {
    // In grid space: dir.x is X, dir.y is Z
    let angle = (dir.x as f32).atan2(dir.y as f32);
    Quat::from_rotation_y(angle)
}

fn corner_to_rotation(incoming: IVec2, outgoing: IVec2) -> Quat {
    match (incoming, outgoing) {
        // Example: Coming from South (0, 1) and turning Right to East (1, 0)
        (IVec2 { x: 0, y: 1 }, IVec2 { x: 1, y: 0 }) => Quat::from_rotation_y(0.0),
        // Add other turn permutations (North-to-East, South-to-West, etc.) as needed
        _ => Quat::IDENTITY,
    }
}
