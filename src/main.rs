use bevy::{log::LogPlugin, prelude::*};
#[cfg(feature = "dev_tools")]
use bevy_devtools::*;

mod asset_loader;
mod camera;
#[cfg(feature = "dev_tools")]
mod dev;
mod factory;
mod preview;
mod road;
mod states;
mod terrain;
mod ui;

fn main() {
    #[cfg(feature = "dev_tools")]
    setup_logger();

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.build().disable::<LogPlugin>(),
        factory::FactoryPlugin,
        camera::CameraPlugin,
        road::RoadPlugin,
        bevy_terrain::TerrainPlugin::new(4),
        MeshPickingPlugin,
        preview::PreviewPlugin,
        asset_loader::AssetLoaderPlugin,
    ))
    .init_state::<states::BuildSelection>();

    #[cfg(feature = "dev_tools")]
    app.add_plugins((tui::TUIPlugin, dev::DevPlugin));

    app.run();
}
