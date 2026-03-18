use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use std::{collections::HashMap, sync::Arc};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
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
    inbound: Vec<FactoryId>,
    outbound: Vec<FactoryId>,
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
