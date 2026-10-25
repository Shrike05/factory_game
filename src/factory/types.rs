use crate::factory::*;
use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Resource)]
pub struct FactoryMap {
    pub shapes: HashMap<FactoryType, Box<[GridPos]>>,
}

#[derive(Message, Clone, Copy)]
pub struct RemoveFactoryMessage {
    pub entity: Entity,
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

    pub fn get_grid_tiles(
        &self,
        pos: &GridPos,
        factory_type: &FactoryType,
    ) -> Result<Vec<GridPos>, String> {
        let shape = self
            .shapes
            .get(factory_type)
            .ok_or_else(|| format!("Factory '{:?}' doesn't have a shape", factory_type))?;
        Ok(shape.iter().map(|x| *x + *pos).collect())
    }
}
