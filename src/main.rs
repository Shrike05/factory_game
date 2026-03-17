use bevy::{log::LogPlugin, prelude::*};

mod camera;
mod factory;
mod preview;
mod road;
mod states;
mod terrain;
mod tui;
mod ui;

fn main() {
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();
    tui_logger::set_default_level(tui_logger::LevelFilter::Info);

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
