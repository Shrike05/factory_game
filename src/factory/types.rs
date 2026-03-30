use bevy::prelude::*;
use bevy_terrain::*;

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

    pub fn as_string(&self) -> String {
        String::from_utf8_lossy(self.0.split(|&b| b == 0).next().unwrap_or(&[])).into_owned()
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Factory {
    pub name: FactoryName,
    pub origin: GridPos,
}

#[derive(Message)]
pub struct NewFactoryEvent {
    pub pos: GridPos,
    pub factory_name: FactoryName,
}

impl NewFactoryEvent {
    pub fn new(pos: GridPos, factory_name: FactoryName) -> NewFactoryEvent {
        NewFactoryEvent { pos, factory_name }
    }
}

impl Factory {
    pub fn new(name: FactoryName, origin: GridPos) -> Factory {
        Factory { name, origin }
    }
}
