use crate::factory::{FactoryMap, FactoryType, NewFactoryEvent};
use crate::globals::*;
use crate::states::BuildSelection;
use crate::terrain::*;
use bevy::prelude::*;

pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(0.5, 0.5)));
    let material = materials.add(Color::srgb(0.25, 0.25, 0.25));
    let hover_material = materials.add(Color::srgb(0.1, 0.1, 0.1));

    for i in 0..(SIZE * SIZE) {
        let pos = grid_to_world(&index_to_grid(i as u32));

        commands.spawn_scene(create_tile(pos, material.clone(), hover_material.clone()));
    }
}

pub fn create_tile(
    pos: Vec3,
    normal_material_handle: Handle<StandardMaterial>,
    hover_material_handle: Handle<StandardMaterial>,
) -> impl Scene {
    bsn! {
        Mesh3d(asset_value(Plane3d::new(Vec3::new(0., 1., 0.), Vec2::new(0.5, 0.5))))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(0.25, 0.25,0.25)))
        Transform::from_xyz(pos.x, pos.y, pos.z)

        on(move |event: On<PointerOver>, mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Transform)>, mut hovered_tile: ResMut<HoveredTile>| {
            if let Ok((mut material, transform)) = query.get_mut(event.event_target()) {
                material.0 = hover_material_handle.clone();
                hovered_tile.pos = world_to_grid(&transform.translation);
                hovered_tile.hovering = true;
            }
        })

        on(move |event: On<PointerOut>, mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Transform)>, mut hovered_tile: ResMut<HoveredTile>| {
            if let Ok((mut material, transform)) = query.get_mut(event.event_target()) {
                material.0 = normal_material_handle.clone();
                hovered_tile.pos = world_to_grid(&transform.translation);
                hovered_tile.hovering = false;
            }
        })

        on(move |event: On<PointerClick>, query: Query<&Transform>, mut msg: MessageWriter<NewFactoryEvent>, build_selection: Res<State<BuildSelection>>| {
            if let Ok(transform) = query.get(event.event_target()) &&  let BuildSelection::Factory(fac_type) = **build_selection{
                msg.write(NewFactoryEvent { pos: world_to_grid(&transform.translation), factory_type: fac_type });
            }
        })
    }
}
