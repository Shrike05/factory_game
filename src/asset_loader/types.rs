use bevy::prelude::*;
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct GameDef {
    pub factory: Option<FactoryDef>,
}

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct FactoryDef {
    pub name: String,
    pub shape: Vec<UVec2>,
    pub recipe_book: Vec<RecipeDef>,
}

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct RecipeDef {
    recipe: String,
    outputs: Vec<String>,
}
