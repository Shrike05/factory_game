use crate::asset_loader::{loader::*, types::*};
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<GameDef>();
        app.add_plugins(TomlAssetPlugin::<GameDef>::new(&["toml"]));
        app.add_systems(Startup, setup);
        app.add_systems(Update, check_and_init_lock);
    }
}
