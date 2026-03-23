use crate::factory::FactoryName;
use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum BuildSelection {
    Factory(FactoryName),
    Road,
    #[default]
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InFactoryMode {
    True,
}

impl ComputedStates for InFactoryMode {
    type SourceStates = BuildSelection;

    fn compute(sources: BuildSelection) -> Option<Self> {
        match sources {
            BuildSelection::Factory(_) => Some(Self::True),
            _ => None,
        }
    }
}
