use bevy::prelude::*;

mod camera;
mod factory;
mod preview;
mod road;
mod states;
mod terrain;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            factory::FactoryPlugin,
            camera::CameraPlugin,
            road::RoadPlugin,
            terrain::TerrainPlugin,
            MeshPickingPlugin,
            preview::PreviewPlugin,
        ))
        .init_state::<states::BuildSelection>()
        .run();
}
