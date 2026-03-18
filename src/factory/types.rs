use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FactoryAttributes {
    pub fac_type: FactoryType,
    pub shape: &'static [GridPos],
}

pub static FACTORY_ATTRIBUTES: [FactoryAttributes; 3] = [
    FactoryAttributes {
        fac_type: FactoryType::Empty,
        shape: &[GridPos::new(0, 0)],
    },
    FactoryAttributes {
        fac_type: FactoryType::Sink,
        shape: &[GridPos::new(0, 0)],
    },
    FactoryAttributes {
        fac_type: FactoryType::Source,
        shape: &[GridPos::new(0, 0)],
    },
];

pub fn get_grid_tiles(pos: &GridPos, factory_type: &FactoryType) -> Vec<GridPos> {
    let shape = FACTORY_ATTRIBUTES[*factory_type as usize].shape;
    shape.iter().map(|x| *x + *pos).collect()
}

pub fn get_factory_attributes(factory_type: &FactoryType) -> FactoryAttributes {
    FACTORY_ATTRIBUTES[*factory_type as usize]
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
#[repr(usize)]
pub enum FactoryType {
    #[default]
    Empty,
    Sink,
    Source,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FactoryId(Entity);

impl FactoryId {
    pub fn get(&self) -> &Entity {
        &self.0
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    id: FactoryId,
    pub origin: GridPos,
    pub factory_type: FactoryType,
    pub inbound: Vec<FactoryId>,
    pub outbound: Vec<FactoryId>,
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct FactoryAssets {
    pub meshes: HashMap<FactoryType, Handle<Mesh>>,
    pub materials: HashMap<FactoryType, Handle<StandardMaterial>>,
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

impl Factory {
    fn new(id: Entity, origin: GridPos, factory_type: FactoryType) -> Factory {
        Factory {
            id: FactoryId(id),
            origin,
            factory_type,
            inbound: vec![],
            outbound: vec![],
        }
    }

    pub fn spawn<'a>(
        commands: &'a mut Commands,
        origin: GridPos,
        factory_type: FactoryType,
    ) -> EntityCommands<'a> {
        let mut entity_commands: EntityCommands<'a> = commands.spawn_empty();
        let id = entity_commands.id();
        entity_commands.insert(Factory::new(id, origin, factory_type));
        entity_commands
    }
}
