use std::fs;
use std::path::Path;

use bevy::prelude::*;

use crate::factory::types::{FactoryDef, FactoryDefs};
use crate::factory::{types::Factory, *};
use crate::globals::*;
use crate::terrain::BuildabilityMap;

pub fn load_factory_defs(mut factory_defs: ResMut<FactoryDefs>) {
    let folder_path = Path::new("./assets/defs/");

    let mut defs: Vec<FactoryDef> = Vec::new();

    for entry in fs::read_dir(folder_path).expect("Couldn't read folder") {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let contents = fs::read_to_string(&path).expect("Couldn't get string content");
            let factory_def: FactoryDef = toml::from_str(&contents).expect("Couldn't parse toml");
            defs.push(factory_def);
        }
    }

    factory_defs.defs = defs;
}

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
