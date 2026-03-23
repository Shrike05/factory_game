use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct GameDef {
    pub factory: Option<FactoryDef>,
}

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct FactoryDef {
    pub name: String,
    pub value: u32,
}
