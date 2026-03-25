use bevy::prelude::*;

use crate::{
    factory::{systems::*, *},
    states,
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FactoryMap::init_factory_map());
        app.add_systems(Startup, create_factory_assets);
        app.add_systems(
            Update,
            (spawn_factories, build_factory_event).run_if(in_state(states::InFactoryMode::True)),
        );
        app.add_message::<NewFactoryEvent>();
    }
}
