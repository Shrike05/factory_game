use crate::asset_loader::types::*;
use bevy::{asset::LoadedFolder, prelude::*};
use std::sync::OnceLock;

pub static DEFS: OnceLock<Vec<GameDef>> = OnceLock::new();

#[derive(Resource)]
pub struct FolderHandle(Handle<LoadedFolder>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asset_handle = asset_server.load_folder("defs");
    commands.insert_resource(FolderHandle(asset_handle));
}

pub fn check_and_init_lock(
    folder_handle: Res<FolderHandle>,
    folders: Res<Assets<LoadedFolder>>,
    game_defs: Res<Assets<GameDef>>,
) {
    if let Some(folder) = folders.get(&folder_handle.0) {
        let items: Vec<GameDef> = folder
            .handles
            .iter()
            .filter_map(|untyped_handle| {
                // Convert UntypedHandle -> AssetId<GameDef>
                let id = untyped_handle.id().typed::<GameDef>();
                game_defs.get(id)
            })
            .cloned()
            .collect();

        let _ = DEFS.set(items);
    }
}
