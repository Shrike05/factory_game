use crate::{
    asset_loader::types::*,
    factory::{FACTORY_ATTRIBUTES, FactoryAttributes, FactoryName},
};
use bevy::{asset::LoadedFolder, prelude::*};
use log::info;
use std::{collections::HashMap, sync::OnceLock};

pub static DEFS: OnceLock<Vec<FactoryDef>> = OnceLock::new();

#[derive(Resource)]
pub struct FolderHandle(Handle<LoadedFolder>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setup");
    let asset_handle = asset_server.load_folder("defs/factories");
    commands.insert_resource(FolderHandle(asset_handle));
}

pub fn check_and_init_lock(
    folder_handle: Res<FolderHandle>,
    folders: Res<Assets<LoadedFolder>>,
    game_defs: Res<Assets<FactoryDef>>,
    mut next_load_state: ResMut<NextState<LoadState>>,
) {
    if let Some(folder) = folders.get(&folder_handle.0) {
        let items: Vec<FactoryDef> = folder
            .handles
            .iter()
            .filter_map(|untyped_handle| {
                // Convert UntypedHandle -> AssetId<GameDef>
                let id = untyped_handle.id().typed::<FactoryDef>();
                game_defs.get(id)
            })
            .cloned()
            .collect();

        let mut hash_map: HashMap<FactoryName, FactoryAttributes> = HashMap::new();

        for item in &items {
            let shape = item.shape.clone().into_boxed_slice();
            let name = FactoryName::from_string(&item.name);
            hash_map.insert(
                name,
                FactoryAttributes::new(name, shape, item.mesh.clone(), item.material.clone()),
            );
        }

        let _ = FACTORY_ATTRIBUTES.set(hash_map);

        let _ = DEFS.set(items);

        info!("NextState");
        next_load_state.set(LoadState::Ready);
    }
}
