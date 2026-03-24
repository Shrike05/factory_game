use std::collections::HashMap;

use bevy::prelude::*;

use crate::factory::*;
use crate::terrain::BuildabilityMap;

pub fn create_factory_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::new(1., 1., 1.));
    let w_material = materials.add(Color::srgb(1., 1., 1.));
    let r_material = materials.add(Color::srgb(1., 0., 0.));
    let g_material = materials.add(Color::srgb(0., 1., 0.));

    let mut meshes = HashMap::new();
    for (fac_name, fac_attrib) in FACTORY_ATTRIBUTES
        .get()
        .expect("Factory Defs don't exist yet")
        .iter()
    {
        let fac_mesh = match fac_attrib.mesh.as_str() {
            "Cube" => mesh.clone(),
            _ => {
                info!(
                    "Couldn't find Mesh: {}, Assagning default mesh",
                    fac_attrib.mesh
                );
                mesh.clone()
            }
        };
        meshes.insert(*fac_name, fac_mesh);
    }
    let mut materials = HashMap::new();
    for (fac_name, fac_attrib) in FACTORY_ATTRIBUTES
        .get()
        .expect("Factory Defs don't exist yet")
        .iter()
    {
        let fac_material = match fac_attrib.material.as_str() {
            "red" => r_material.clone(),
            "green" => g_material.clone(),
            "white" => w_material.clone(),
            _ => {
                info!(
                    "Couldn't find Mesh: {}, Assigning default white material",
                    fac_attrib.material
                );
                w_material.clone()
            }
        };
        materials.insert(*fac_name, fac_material);
    }

    commands.insert_resource(FactoryAssets { meshes, materials });
}

pub fn spawn_factories(
    mut commands: Commands,
    fac_assets: Res<FactoryAssets>,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
) {
    for message in msg.read() {
        let mut factory = Factory::spawn(
            &mut commands,
            message.pos,
            FactoryName::from_string("Empty"),
        );

        factory.insert((
            Mesh3d(
                fac_assets
                    .meshes
                    .get(&FactoryName::from_string("Empty"))
                    .unwrap()
                    .clone(),
            ),
            MeshMaterial3d(
                fac_assets
                    .materials
                    .get(&message.factory_name)
                    .unwrap()
                    .clone(),
            ),
            Transform::from_xyz(message.pos.x as f32, 0., message.pos.y as f32),
        ));

        let shape = &get_factory_attributes(&message.factory_name).shape;
        for offset in shape {
            build_map
                .set_real(message.pos + offset, true)
                .expect("Couldn't set factory to the build_map");
        }
    }
}
