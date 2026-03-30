use bevy::prelude::*;
use bevy_defs_loader::Def;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct FactoryDef {
    pub name: String,
    pub shape: Vec<UVec2>,
    pub recipe_book: Vec<RecipeDef>,
}

impl Def for FactoryDef {}

#[derive(Asset, TypePath, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipeDef {
    recipe: String,
    outputs: Vec<String>,
}
