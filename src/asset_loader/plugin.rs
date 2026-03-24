use crate::asset_loader::{loader::*, types::*};
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<FactoryDef>();
        app.insert_state(LoadState::Loading);
        app.add_plugins(TomlAssetPlugin::<FactoryDef>::new(&["toml"]));
        app.add_systems(PreStartup, setup);
        app.add_systems(
            Update,
            check_and_init_lock.run_if(in_state(LoadState::Loading)),
        );
    }
}
