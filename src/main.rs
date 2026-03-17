use bevy::{log::LogPlugin, prelude::*};
use bevy_tui::*;

mod camera;
mod factory;
mod preview;
mod road;
mod states;
mod terrain;
mod ui;

fn main() {
    setup_logger();

    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<LogPlugin>(),
            factory::FactoryPlugin,
            camera::CameraPlugin,
            road::RoadPlugin,
            terrain::TerrainPlugin,
            MeshPickingPlugin,
            preview::PreviewPlugin,
            tui::TUIPlugin,
        ))
        .init_state::<states::BuildSelection>()
        .run();
}
