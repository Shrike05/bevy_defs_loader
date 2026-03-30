use crate::*;
use bevy::{asset::LoadedFolder, prelude::*};

pub fn setup<T: Def>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path: Res<DefsFolder<T>>,
) {
    let asset_handle = asset_server.load_folder(path.0);
}

pub fn check_and_init_lock<T: Def>(
    folder_handle: Res<DefsHandle<T>>,
    folders: Res<Assets<LoadedFolder>>,
    game_defs: Res<Assets<T>>,
    mut defs: ResMut<LoadedDefs<T>>,
) {
    if let Some(folder) = folders.get(&folder_handle.0) {
        let items: Vec<T> = folder
            .handles
            .iter()
            .filter_map(|untyped_handle| {
                // Convert UntypedHandle -> AssetId<GameDef>
                let id = untyped_handle.id().typed::<T>();
                game_defs.get(id)
            })
            .cloned()
            .collect();

        defs.0 = items;
    }
}
