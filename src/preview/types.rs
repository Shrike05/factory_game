use bevy::prelude::*;

#[derive(Component, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct PreviewFactory;

#[derive(Component, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct PreviewRoad;

#[derive(Resource, Debug, PartialEq, Eq, Clone)]
pub struct PreviewAssets {
    pub normal_mat: Handle<StandardMaterial>,
    pub warning_mat: Handle<StandardMaterial>,
}
