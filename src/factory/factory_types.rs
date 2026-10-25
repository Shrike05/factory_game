use crate::factory::*;
use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash, ValueEnum)]
pub enum FactoryType {
    #[default]
    Empty,
    Sink,
    Source,
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct Sink;

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct Source;

#[derive(Component, Clone, PartialEq, Debug, Default)]
pub struct Factory {
    pub origin: GridPos,
    pub factory_type: FactoryType,
    pub input: Vec<(Item, f32)>,
    pub output: Vec<(Item, f32)>,
}

impl Factory {
    pub fn new(origin: GridPos, factory_type: FactoryType) -> Factory {
        let (input, output) = match factory_type {
            FactoryType::Empty => (vec![(Item::Rock, 2.)], vec![(Item::Iron, 1.)]),
            FactoryType::Source => (vec![], vec![(Item::Rock, 1.)]),
            FactoryType::Sink => (vec![(Item::Iron, 1.)], vec![]),
        };

        Factory {
            origin,
            factory_type,
            input,
            output,
        }
    }
}
