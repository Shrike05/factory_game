use std::fs;
use std::path::Path;

use bevy::prelude::*;

use crate::factory::types::{FactoryConnections, FactoryDef, FactoryDefs};
use crate::factory::{types::Factory, *};
use crate::globals::*;
use crate::road::BuildRoadMessage;
use crate::states::BuildSelection;
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

pub fn spawn_factories(mut commands: Commands, mut msg: MessageReader<NewFactoryEvent>) {
    for message in msg.read() {
        let mut entity = commands.spawn_scene(create_factory(message.pos, message.factory_type));

        let entity_id = entity.id();
        let observer = move |event: On<PointerClick>,
                             query: Query<&Transform>,
                             mut msg: MessageWriter<BuildRoadMessage>,
                             build_selection: Res<State<BuildSelection>>| {
            if let Ok(transform) = query.get(event.event_target()) {
                match **build_selection {
                    BuildSelection::Road => {
                        msg.write(BuildRoadMessage::End(
                            world_to_grid(&transform.translation),
                            entity_id,
                        ));
                    }
                    _ => {
                        msg.write(BuildRoadMessage::Start(
                            world_to_grid(&transform.translation),
                            entity_id,
                        ));
                    }
                }
            }
        };

        entity.observe(observer);
    }
}

pub fn create_factory(pos: GridPos, fac_type: FactoryType) -> impl Scene {
    let factory = Factory::new(pos, fac_type);
    bsn! {
        Factory::new(pos, fac_type)
        Mesh3d(asset_value(Cuboid::new(0.8, 1., 0.8)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 1., 1.)))
        Transform::from_xyz(factory.origin.x as f32, 0., factory.origin.y as f32)
        FactoryConnections::default()
    }
}
