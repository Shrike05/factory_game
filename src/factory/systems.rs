use bevy::prelude::*;

use crate::factory::{types::Factory, *};
use crate::terrain::BuildabilityMap;

pub fn create_factory_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FactoryAssets {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::srgb(1., 1., 1.)),
    });
}

pub fn spawn_factories(
    mut commands: Commands,
    fac_assets: Res<FactoryAssets>,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
    fac_map: Res<FactoryMap>,
) {
    for message in msg.read() {
        let factory = Factory::new(message.pos, message.factory_type);

        commands.spawn((
            factory,
            Mesh3d(fac_assets.mesh.clone()),
            MeshMaterial3d(fac_assets.material.clone()),
            Transform::from_xyz(factory.origin.x as f32, 0., factory.origin.y as f32),
        ));

        let shape = fac_map.shapes[&factory.factory_type].clone();
        for offset in shape {
            //let i = build_map.pos_to_index(factory.origin.x + offset.x, factory.origin.y + offset.y);
            build_map.set_real(factory.origin + offset, true);
        }
    }
}
