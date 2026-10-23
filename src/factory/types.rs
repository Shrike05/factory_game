use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Item {
    #[default]
    Rock,
    Iron,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct FactoryDefs {
    pub defs: Vec<FactoryDef>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FactoryDef {
    name: String,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
pub enum FactoryType {
    #[default]
    Empty,
    Sink,
    Source,
}

#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Factory {
    pub origin: GridPos,
    pub factory_type: FactoryType,
    pub input: Vec<(Item, f32)>,
    pub output: Vec<(Item, f32)>,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct Sink;

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct Source;

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
        let shape = self.shapes.get(factory_type).expect(&format!(
            "Factory '{:?}' doesn't have a shape",
            factory_type
        ));

        shape.iter().map(|x| *x + *pos).collect()
    }
}

impl Factory {
    pub fn new(origin: GridPos, factory_type: FactoryType) -> Factory {
        let (input, output) = match factory_type {
            FactoryType::Empty => (vec![(Item::Rock, 2.)], vec![(Item::Iron, 1.)]),
            FactoryType::Source => (vec![], vec![(Item::Rock, 1.)]),
            FactoryType::Sink => (vec![(Item::Iron, 1.)], vec![]),
        };

        Factory {
            origin,
            factory_type,
            input,
            output,
        }
    }
}
