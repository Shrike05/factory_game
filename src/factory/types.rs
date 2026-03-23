use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactoryAttributes {
    pub name: FactoryName,
    pub shape: Box<[GridPos]>,
}

impl FactoryAttributes {
    pub fn new(name: FactoryName, shape: Box<[GridPos]>) -> Self {
        FactoryAttributes { name, shape }
    }
}

pub static FACTORY_ATTRIBUTES: OnceLock<HashMap<FactoryName, FactoryAttributes>> = OnceLock::new();

pub fn get_grid_tiles(pos: &GridPos, factory_name: &FactoryName) -> Vec<GridPos> {
    println!("{:?}", FACTORY_ATTRIBUTES);
    let shape = &FACTORY_ATTRIBUTES
        .get()
        .expect("Factory Defs not ready yet")
        .get(factory_name)
        .expect("factory def does not exist")
        .shape;
    shape.iter().map(|x| *x + *pos).collect()
}

pub fn get_factory_attributes(factory_name: &FactoryName) -> &FactoryAttributes {
    FACTORY_ATTRIBUTES
        .get()
        .expect("Factory Defs not ready yet")
        .get(factory_name)
        .expect("factory def does not exist")
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactoryName([u8; 32]);

impl FactoryName {
    pub fn new(name: [u8; 32]) -> Self {
        FactoryName(name)
    }

    pub fn from_string<T: Into<String>>(name: T) -> Self {
        let name_string = name.into();
        let mut data = [0u8; 32];
        let n = name_string.as_bytes();
        let len = n.len().min(32);
        data[..len].copy_from_slice(&n[..len]);
        FactoryName(data)
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    id: FactoryId,
    name: FactoryName,
    pub origin: GridPos,
    pub inbound: Vec<FactoryId>,
    pub outbound: Vec<FactoryId>,
}

#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct FactoryAssets {
    pub meshes: HashMap<FactoryName, Handle<Mesh>>,
    pub materials: HashMap<FactoryName, Handle<StandardMaterial>>,
}

#[derive(Message)]
pub struct NewFactoryEvent {
    pub pos: GridPos,
    pub factory_type: FactoryName,
}

impl NewFactoryEvent {
    pub fn new(pos: GridPos, factory_type: FactoryName) -> NewFactoryEvent {
        NewFactoryEvent { pos, factory_type }
    }
}

impl Factory {
    fn new(id: Entity, name: FactoryName, origin: GridPos) -> Factory {
        Factory {
            id: FactoryId(id),
            name,
            origin,
            inbound: vec![],
            outbound: vec![],
        }
    }

    pub fn spawn<'a>(
        commands: &'a mut Commands,
        origin: GridPos,
        name: FactoryName,
    ) -> EntityCommands<'a> {
        let mut entity_commands: EntityCommands<'a> = commands.spawn_empty();
        let id = entity_commands.id();
        entity_commands.insert(Factory::new(id, name, origin));
        entity_commands
    }

    pub fn get_name(&self) -> &FactoryName {
        &self.name
    }
}
