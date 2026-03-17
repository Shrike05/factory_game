use bevy::prelude::*;
use clap::ValueEnum;
use std::collections::HashMap;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
pub enum FactoryType {
    #[default]
    Empty,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    pub origin: IVec2,
    pub factory_type: FactoryType,
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct FactoryAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct FactoryMap {
    pub shapes: HashMap<FactoryType, Box<[IVec2]>>,
}

#[derive(Message)]
pub struct NewFactoryEvent {
    pub pos: IVec2,
    pub factory_type: FactoryType,
}

impl NewFactoryEvent {
    pub fn new(pos: IVec2, factory_type: FactoryType) -> NewFactoryEvent {
        NewFactoryEvent { pos, factory_type }
    }
}

impl FactoryMap {
    pub fn init_factory_map() -> FactoryMap {
        let mut factory_map = HashMap::new();
        factory_map.insert(
            FactoryType::Empty,
            vec![IVec2::new(0, 0)].into_boxed_slice(),
        );

        FactoryMap {
            shapes: factory_map,
        }
    }
}

impl Factory {
    pub fn new(origin: IVec2, factory_type: FactoryType) -> Factory {
        Factory {
            origin,
            factory_type,
        }
    }
}
