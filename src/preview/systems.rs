use crate::preview::types::*;
use crate::road::{Road, RoadAssets, RoadConstructor};
use crate::terrain::BuildSelection;
use crate::{
    factory::{FactoryAssets, FactoryMap},
    terrain::{BuildabilityMap, HoveredTile},
};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub fn init_preview(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transp_mat = materials.add(Color::srgba(1., 1., 1., 0.3));
    let warning_mat = materials.add(Color::srgba(1., 0., 0., 0.3));

    commands.insert_resource(PreviewAssets {
        normal_mat: transp_mat.clone(),
        warning_mat,
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::default())),
        MeshMaterial3d(transp_mat),
        Transform::from_xyz(0., 0., 0.),
        Visibility::Visible,
        Pickable::IGNORE,
        PreviewFactory,
    ));
}

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
    pub build_select: Res<'w, BuildSelection>,
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

    let path_vec = Road::create_candidate_road(
        &params.road_constructor.get_start().unwrap(),
        &params
            .road_constructor
            .get_end()
            .unwrap_or(params.hovered_tile.pos),
        *params.build_map,
    )
    .unwrap_or(vec![]);
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
                if !params.build_map.overlaps(&vec![IVec2::new(
                    tran.translation.x as i32,
                    tran.translation.z as i32,
                )]) {
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

pub fn preview_factory(
    mut pre_query: Query<
        (
            &mut Mesh3d,
            &mut MeshMaterial3d<StandardMaterial>,
            &mut Transform,
            &mut Visibility,
        ),
        With<PreviewFactory>,
    >,
    fac_assets: Res<FactoryAssets>,
    fac_map: Res<FactoryMap>,
    build_map: Res<BuildabilityMap>,
    prev_mat: Res<PreviewAssets>,
    build_select: Res<BuildSelection>,
    hovered_tile: Res<HoveredTile>,
) {
    let (mut mesh, mut mat, mut tran, mut vis) = pre_query
        .iter_mut()
        .next()
        .expect("Preview not initialized");

    if let BuildSelection::Factory(fac_type) = *build_select {
        mesh.0 = fac_assets.mesh.clone();

        let tiles = fac_map.shapes[&fac_type]
            .iter()
            .map(|x| {
                IVec2::new(
                    x.x + tran.translation.x as i32,
                    x.y + tran.translation.z as i32,
                )
            })
            .collect();
        if !build_map.overlaps(&tiles) {
            mat.0 = prev_mat.normal_mat.clone();
        } else {
            mat.0 = prev_mat.warning_mat.clone();
        }
    }

    if !hovered_tile.hovering {
        *vis = Visibility::Hidden;
    } else {
        *vis = Visibility::Visible;
    }

    tran.translation.x = hovered_tile.pos.x as f32;
    tran.translation.z = hovered_tile.pos.y as f32;
}
