use std::collections::HashMap;

use bevy::prelude::*;

use crate::factory::{types::Factory, *};
use crate::globals::*;
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
    meshes.insert(FactoryType::Empty, mesh);
    let mut materials = HashMap::new();
    materials.insert(FactoryType::Empty, w_material);
    materials.insert(FactoryType::Source, g_material);
    materials.insert(FactoryType::Sink, r_material);
    commands.insert_resource(FactoryAssets { meshes, materials });
}

pub fn spawn_factories(
    mut commands: Commands,
    fac_assets: Res<FactoryAssets>,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
) {
    for message in msg.read() {
        let mut factory = Factory::spawn(&mut commands, message.pos, message.factory_type);

        factory.insert((
            Mesh3d(fac_assets.meshes.get(&FactoryType::Empty).unwrap().clone()),
            MeshMaterial3d(
                fac_assets
                    .materials
                    .get(&message.factory_type)
                    .unwrap()
                    .clone(),
            ),
            Transform::from_xyz(message.pos.x as f32, 0., message.pos.y as f32),
        ));

        let shape: &[GridPos] = get_factory_attributes(&message.factory_type).shape;
        for offset in shape {
            build_map
                .set_real(message.pos + offset, true)
                .expect("Couldn't set factory to the build_map");
        }
    }
}
