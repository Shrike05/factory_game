use crate::factory::FactoryType;
use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum BuildSelection {
    Factory(FactoryType),
    Road,
    #[default]
    None,
}
