use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    factory::{systems::*, *},
    states,
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FactoryMap::init_factory_map());
        app.insert_resource(Production::default());
        app.add_systems(
            Update,
            spawn_factories.run_if(in_state(states::InFactoryMode::True)),
        );
        app.add_systems(
            Update,
            production_cycle.run_if(on_timer(Duration::from_secs(1))),
        );
        app.add_systems(Update, remove_factory);
        app.add_message::<NewFactoryEvent>();
        app.add_message::<RemoveFactoryMessage>();
    }
}
