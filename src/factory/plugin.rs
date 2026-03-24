use bevy::prelude::*;

use crate::{
    LoadState,
    factory::{
        NewFactoryEvent,
        systems::{create_factory_assets, spawn_factories},
    },
    states,
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LoadState::Ready), create_factory_assets);
        app.add_systems(
            Update,
            spawn_factories
                .run_if(in_state(states::InFactoryMode::True).and(in_state(LoadState::Ready))),
        );
        app.add_message::<NewFactoryEvent>();
    }
}
