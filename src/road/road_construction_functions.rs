use crate::globals::*;
use crate::terrain::BuildabilityMap;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use std::f32::consts::FRAC_PI_2;

pub const SEARCH_DEPTH: u32 = 100;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PathSegmentType {
    Start { outgoing: IVec2 },
    End { incoming: IVec2 },
    Straight { direction: IVec2 },
    Corner { incoming: IVec2, outgoing: IVec2 },
    SingleNode,
}

pub fn find_path_segment(
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
        current_depth += 1;
    }

    None
}

pub fn create_candidate_road(
    points: &[GridPos],
    filter_map: BuildabilityMap,
) -> Option<Vec<GridPos>> {
    if points.len() < 2 {
        return None;
    }

    let mut full_path: Vec<GridPos> = Vec::new();
    let mut global_visited: HashSet<GridPos> = HashSet::new();

    for window in points.windows(2) {
        let p_start = IVec2::new(window[0].x as i32, window[0].y as i32);
        let p_end = IVec2::new(window[1].x as i32, window[1].y as i32);

        let segment = find_path_segment(p_start, p_end, &filter_map, &global_visited)?;

        if full_path.is_empty() {
            global_visited.extend(segment.iter().cloned());
            full_path.extend(segment);
        } else {
            let slice = &segment[1..];
            global_visited.extend(slice.iter().cloned());
            full_path.extend(slice.iter().cloned());
        }
    }

    Some(full_path)
}

pub fn build_road_scene(
    points: &[GridPos],
    buildability_map: &BuildabilityMap,
    road_color: Option<Color>,
) -> Option<Vec<(UVec2, Box<dyn Scene>)>> {
    let color = road_color.unwrap_or(Color::srgb(0.54, 0.17, 0.14));
    let path = create_candidate_road(points, *buildability_map)?;
    let mut scenes = Vec::new();

    let analysis = analyze_path(&path);
    if analysis.len() < 2 {
        return None;
    }

    for (pos, path_type) in analysis[1..analysis.len() - 1].iter().copied() {
        //for (pos, path_type) in analysis {
        let world_pos = Vec3::new(pos.x as f32, 0., pos.y as f32);

        let segment_scene: Box<dyn Scene> = match path_type {
            PathSegmentType::Start { outgoing }
            | PathSegmentType::Straight {
                direction: outgoing,
            } => {
                let rotation = direction_to_rotation(outgoing);
                Box::new(bsn! {
                    Transform::from_translation(world_pos).with_rotation(rotation)
                    @straight_path_mesh(color)
                })
            }
            PathSegmentType::Corner { incoming, outgoing } => {
                let rotation = corner_to_rotation(incoming, outgoing);
                Box::new(bsn! {
                    Transform::from_translation(world_pos).with_rotation(rotation)
                    @corner_mesh(color)
                })
            }
            PathSegmentType::End { incoming } => {
                let rotation = direction_to_rotation(incoming);
                Box::new(bsn! {
                    Transform::from_translation(world_pos).with_rotation(rotation)
                    @straight_path_mesh(color)
                })
            }
            PathSegmentType::SingleNode => continue,
        };

        scenes.push((pos, segment_scene));
    }

    Some(scenes)
}

pub fn direction_to_rotation(dir: IVec2) -> Quat {
    // In grid space: dir.x is X, dir.y is Z
    let angle = (dir.x as f32).atan2(dir.y as f32);
    Quat::from_rotation_y(angle)
}

pub fn corner_to_rotation(incoming: IVec2, outgoing: IVec2) -> Quat {
    match (incoming, outgoing) {
        (IVec2 { x: 0, y: -1 }, IVec2 { x: 1, y: 0 })
        | (IVec2 { x: -1, y: 0 }, IVec2 { x: 0, y: 1 }) => Quat::from_rotation_y(-FRAC_PI_2),

        (IVec2 { x: 1, y: 0 }, IVec2 { x: 0, y: 1 })
        | (IVec2 { x: 0, y: -1 }, IVec2 { x: -1, y: 0 }) => Quat::from_rotation_y(FRAC_PI_2 * 2.),

        (IVec2 { x: 0, y: 1 }, IVec2 { x: -1, y: 0 })
        | (IVec2 { x: 1, y: 0 }, IVec2 { x: 0, y: -1 }) => Quat::from_rotation_y(FRAC_PI_2),

        (IVec2 { x: -1, y: 0 }, IVec2 { x: 0, y: -1 })
        | (IVec2 { x: 0, y: 1 }, IVec2 { x: 1, y: 0 }) => Quat::from_rotation_y(0.),

        _ => Quat::IDENTITY,
    }
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

pub fn straight_path_mesh(color: Color) -> impl Scene {
    bsn! {
        Children[
            Mesh3d(asset_value(Cuboid::new(0.5, 0.2, 1.)))
            MeshMaterial3d<StandardMaterial>(asset_value(color))
        ]
    }
}

pub fn end_mesh(color: Color) -> impl Scene {
    bsn! {
        Children[
            Mesh3d(asset_value(Cuboid::new(0.5, 0.2, 0.5)))
            MeshMaterial3d<StandardMaterial>(asset_value(color))
        ]
    }
}

pub fn corner_mesh(color: Color) -> impl Scene {
    bsn! {
        Children [
            // First arm of the L-bend (e.g., stretching along Z)
            Transform::from_xyz(0.0, 0.0, -0.125)
            Mesh3d(asset_value(Cuboid::new(0.5, 0.2, 0.75)))
            MeshMaterial3d<StandardMaterial>(asset_value(color))
            --
            // Second arm of the L-bend (e.g., branching out along X)
            Transform::from_xyz(0.125, 0.0, 0.0)
            Mesh3d(asset_value(Cuboid::new(0.75, 0.2, 0.5)))
            MeshMaterial3d<StandardMaterial>(asset_value(color))
        ]
    }
}
