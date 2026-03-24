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
    pub material: String,
    pub mesh: String,
}

#[derive(TypePath, Deserialize, Debug, Clone)]
pub struct RecipeDef {
    recipe: String,
    outputs: Vec<String>,
}

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct ItemDef {
    pub name: String,
}

#[derive(States, Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Ready,
}
