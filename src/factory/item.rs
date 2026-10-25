use crate::globals::*;
use bevy::prelude::*;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Item {
    #[default]
    Rock,
    Iron,
}
