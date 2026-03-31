use bevy::{platform::collections::HashMap, prelude::*};
use bevy_terrain::GridPos;

#[derive(Resource, Clone, Debug, Default)]
pub struct TileAttributes {
    factory_io: HashMap<GridPos, bool>,
}

impl TileAttributes {
    pub fn get(&self, position: GridPos) -> Option<&bool> {
        self.factory_io.get(&position)
    }
    pub fn set(&mut self, position: GridPos, value: bool) {
        self.factory_io.insert(position, value);
    }
}
