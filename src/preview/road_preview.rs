use crate::preview::types::*;
use crate::road::{Road, RoadAssets, RoadConstructor};
use crate::states::BuildSelection;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_terrain::*;

#[derive(SystemParam)]
pub struct RoadPreviewParams<'w, 's> {
    pub query: Query<
        'w,
        's,
        (
            &'static mut Mesh3d,
            &'static mut MeshMaterial3d<StandardMaterial>,
            &'static mut Transform,
            &'static mut Visibility,
            &'static PreviewRoad,
        ),
    >,
    pub build_map: Res<'w, BuildabilityMap>,
    pub road_assets: Res<'w, RoadAssets>,
    pub prev_mat: Res<'w, PreviewAssets>,
    pub build_select: Res<'w, State<BuildSelection>>,
    pub road_constructor: Res<'w, RoadConstructor>,
    pub hovered_tile: Res<'w, HoveredTile>,
}

pub fn preview_road(mut commands: Commands, params: RoadPreviewParams) {
    if params.road_constructor.get_start().is_none() {
        for (_, _, _, mut vis, _) in params.query {
            *vis = Visibility::Hidden;
        }
        return;
    }

    let mut points = vec![params.road_constructor.get_start().unwrap()];
    let _ = params
        .road_constructor
        .get_waypoints()
        .iter()
        .map(|&pt| points.push(pt))
        .collect::<Vec<()>>();
    points.push(
        params
            .road_constructor
            .get_end()
            .unwrap_or(params.hovered_tile.pos),
    );

    let road = Road::new(points).expect("Couldn't build Road");

    let path_vec = Road::create_candidate_road(&road, &params.build_map).unwrap_or(vec![]);

    let mut path = path_vec.iter();

    for (mut mesh, mut mat, mut tran, mut vis, _) in params.query {
        if !params.build_select.eq(&BuildSelection::Road) {
            continue;
        }

        mesh.0 = params.road_assets.mesh.clone();
        mat.0 = params.prev_mat.normal_mat.clone();

        match path.next() {
            Some(pos) => {
                tran.translation.x = pos.x as f32;
                tran.translation.z = pos.y as f32;
                if !params
                    .build_map
                    .overlaps(&[world_to_grid(&tran.translation)])
                {
                    mat.0 = params.prev_mat.normal_mat.clone();
                } else {
                    mat.0 = params.prev_mat.warning_mat.clone();
                }

                *vis = Visibility::Visible;
            }
            None => {
                *vis = Visibility::Hidden;
            }
        }
    }

    for pos in path {
        commands.spawn((
            Mesh3d(params.road_assets.mesh.clone()),
            MeshMaterial3d(params.prev_mat.normal_mat.clone()),
            Transform::from_xyz(pos.x as f32, 0., pos.y as f32),
            Visibility::Visible,
            Pickable::IGNORE,
            PreviewRoad,
        ));
    }
}

pub fn stop_build_road(
    mut road_constructor: ResMut<RoadConstructor>,
    mut pre_query: Query<&mut Visibility, With<PreviewRoad>>,
) {
    for mut vis in pre_query.iter_mut() {
        *vis = Visibility::Hidden;
    }

    *road_constructor = RoadConstructor::empty();
}
