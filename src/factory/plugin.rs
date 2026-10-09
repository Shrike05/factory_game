use bevy::prelude::*;

use crate::{
    factory::{
        systems::{load_factory_defs, spawn_factories},
        types::FactoryDefs,
        *,
    },
    states,
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FactoryMap::init_factory_map());
        app.insert_resource(FactoryDefs::default());
        app.add_systems(Startup, load_factory_defs);
        app.add_systems(
            Update,
            spawn_factories.run_if(in_state(states::InFactoryMode::True)),
        );
        app.add_message::<NewFactoryEvent>();
    }
}
