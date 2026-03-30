use bevy::prelude::*;
use bevy_defs_loader::{DefPlugin, DefsLoadState};

use crate::{
    factory::{attributes::*, defs::FactoryDef, factory_preview::*, systems::*, *},
    states::{self, InFactoryMode},
};

pub struct FactoryPlugin;

impl Plugin for FactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DefsLoadState::Ready), create_factory_assets);
        app.add_systems(
            Update,
            (spawn_factories, build_factory_event)
                .run_if(in_state(states::InFactoryMode::True).and(in_state(DefsLoadState::Ready))),
        );
        app.add_message::<NewFactoryEvent>();

        app.add_plugins(DefPlugin::<FactoryDef>::new("defs"));
        app.add_systems(OnEnter(DefsLoadState::Ready), init_attributes);

        app.insert_resource(FactoryMaterials::default());
        app.insert_resource(FactoryMeshes::default());
        app.insert_resource(FactoryShapes::default());

        app.add_systems(Startup, init_factory_preview);
        app.add_computed_state::<InFactoryMode>()
            .add_systems(
                Update,
                preview_factory.run_if(in_state(InFactoryMode::True)),
            )
            .add_systems(OnExit(InFactoryMode::True), stop_preview_factory);
    }
}
