use crate::factory::{FactoryAttribute, FactoryMeshes, FactoryShapes};
use crate::preview::*;
use crate::states::BuildSelection;
use bevy::prelude::*;
use bevy_terrain::*;

pub fn init_factory_preview(
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
        Visibility::Hidden,
        Pickable::IGNORE,
        PreviewFactory,
    ));
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
    fac_meshes: Res<FactoryMeshes>,
    shape_map: Res<FactoryShapes>,
    build_map: Res<BuildabilityMap>,
    prev_mat: Res<PreviewAssets>,
    build_select: Res<State<BuildSelection>>,
    hovered_tile: Res<HoveredTile>,
) {
    let (mut mesh, mut mat, mut tran, mut vis) = pre_query
        .iter_mut()
        .next()
        .expect("Preview not initialized");

    if let BuildSelection::Factory(factory_name) = **build_select {
        mesh.0 = fac_meshes.get(&factory_name).clone();

        let tiles = shape_map.get_grid_tiles(&world_to_grid(&tran.translation), &factory_name);
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

pub fn stop_preview_factory(mut pre_query: Query<&mut Visibility, With<PreviewFactory>>) {
    for mut vis in pre_query.iter_mut() {
        *vis = Visibility::Hidden;
    }
}
