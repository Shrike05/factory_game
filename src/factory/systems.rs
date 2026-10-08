use bevy::prelude::*;

use crate::factory::{types::Factory, *};
use crate::globals::*;
use crate::terrain::BuildabilityMap;

pub fn spawn_factories(
    mut commands: Commands,
    mut msg: MessageReader<NewFactoryEvent>,
    mut build_map: ResMut<BuildabilityMap>,
    fac_map: Res<FactoryMap>,
) {
    for message in msg.read() {
        commands.spawn_scene(create_factory(message.pos, message.factory_type));

        let factory = Factory::new(message.pos, message.factory_type);

        let shape: Box<[GridPos]> = fac_map.shapes[&factory.factory_type].clone();
        for offset in shape {
            build_map
                .set_real(factory.origin + offset, true)
                .expect("Couldn't set factory to the build_map");
        }
    }
}

pub fn create_factory(pos: GridPos, fac_type: FactoryType) -> impl Scene {
    let factory = Factory::new(pos, fac_type);
    bsn! {
        Factory::new(pos, fac_type)
        Mesh3d(asset_value(Cuboid::new(1., 1., 1.)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 1., 1.)))
        Transform::from_xyz(factory.origin.x as f32, 0., factory.origin.y as f32)
    }
}
