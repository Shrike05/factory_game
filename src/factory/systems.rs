use crate::factory::attributes::*;
use crate::factory::defs::FactoryDef;
use crate::factory::types::FactoryName;
use crate::factory::{types::Factory, *};
use crate::states::*;
use bevy::prelude::*;
use bevy_defs_loader::LoadedDefs;
use bevy_terrain::*;

pub fn create_factory_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut fac_meshes: ResMut<FactoryMeshes>,
    mut fac_materials: ResMut<FactoryMaterials>,
    defs: Res<LoadedDefs<FactoryDef>>,
) {
    let cube_mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let w_material = materials.add(Color::srgb(1., 1., 1.));
    let r_material = materials.add(Color::srgb(1., 0., 0.));
    let g_material = materials.add(Color::srgb(0., 1., 0.));

    for def in defs.0.clone() {
        let name = FactoryName::from_string(def.name);

        let mesh = match def.mesh.to_lowercase().as_str() {
            "cube" => &cube_mesh,
            _ => &cube_mesh,
        };

        fac_meshes.set(name, mesh.clone());

        let mat = match def.material.to_lowercase().as_str() {
            "green" => &g_material,
            "red" => &r_material,
            "white" => &w_material,
            _ => &w_material,
        };

        fac_materials.set(name, mat.clone());
    }
}

pub fn spawn_factories(
    mut commands: Commands,
    meshes: Res<FactoryMeshes>,
    materials: Res<FactoryMaterials>,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
    shapes_map: Res<FactoryShapes>,
) {
    for message in msg.read() {
        let factory = Factory::new(message.factory_name, message.pos);

        commands.spawn((
            factory,
            Mesh3d(meshes.get(&factory.name).clone()),
            MeshMaterial3d(materials.get(&factory.name).clone()),
            Transform::from_xyz(factory.origin.x as f32, 0., factory.origin.y as f32),
        ));

        let shape: Box<[GridPos]> = shapes_map.get(&factory.name).clone();
        for offset in shape {
            build_map
                .set_real(factory.origin + offset, true)
                .expect("Couldn't set factory to the build_map");
        }
    }
}

pub fn build_factory_event(
    mut ev: MessageReader<TileClickedMessage>,
    mut fac_writer: MessageWriter<NewFactoryEvent>,
    shape_map: Res<FactoryShapes>,
    build_map: Res<BuildabilityMap>,
    build_selection: Res<State<BuildSelection>>,
) {
    for build_ev in ev.read() {
        let tiles: Vec<GridPos> = shape_map
            .get(&FactoryName::from_string("empty"))
            .iter()
            .map(|x| x + build_ev.get_pos())
            .collect();

        if !build_map.overlaps(&tiles)
            && let BuildSelection::Factory(fac_type) = **build_selection
        {
            fac_writer.write(NewFactoryEvent::new(*build_ev.get_pos(), fac_type));
        }
    }
}
