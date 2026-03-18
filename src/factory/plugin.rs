use bevy::prelude::*;

use crate::{
    factory::{
        systems::{create_factory_assets, spawn_factories},
        *,
    },
    states,
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_factory_assets);
        app.add_systems(
            Update,
            spawn_factories.run_if(in_state(states::InFactoryMode::True)),
        );
        app.add_message::<NewFactoryEvent>();
    }
}
