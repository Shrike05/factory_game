use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use std::collections::HashMap;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
pub enum FactoryType {
    #[default]
    Empty,
    Sink,
    Source,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    pub origin: GridPos,
    pub factory_type: FactoryType,
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct FactoryAssets {
    pub meshes: HashMap<FactoryType, Handle<Mesh>>,
    pub materials: HashMap<FactoryType, Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct FactoryMap {
    pub shapes: HashMap<FactoryType, Box<[GridPos]>>,
}

#[derive(Message)]
pub struct NewFactoryEvent {
    pub pos: GridPos,
    pub factory_type: FactoryType,
}

impl NewFactoryEvent {
    pub fn new(pos: GridPos, factory_type: FactoryType) -> NewFactoryEvent {
        NewFactoryEvent { pos, factory_type }
    }
}

impl FactoryMap {
    pub fn init_factory_map() -> FactoryMap {
        let mut factory_map = HashMap::new();
        factory_map.insert(
            FactoryType::Empty,
            vec![GridPos::new(0, 0)].into_boxed_slice(),
        );
        factory_map.insert(
            FactoryType::Sink,
            vec![GridPos::new(0, 0)].into_boxed_slice(),
        );
        factory_map.insert(
            FactoryType::Source,
            vec![GridPos::new(0, 0)].into_boxed_slice(),
        );

        FactoryMap {
            shapes: factory_map,
        }
    }

    pub fn get_grid_tiles(&self, pos: &GridPos, factory_type: &FactoryType) -> Vec<GridPos> {
        let shape = self
            .shapes
            .get(factory_type)
            .expect("Factory doesn't have a shape");

        shape.iter().map(|x| *x + *pos).collect()
    }
}

impl Factory {
    pub fn new(origin: GridPos, factory_type: FactoryType) -> Factory {
        Factory {
            origin,
            factory_type,
        }
    }
}
