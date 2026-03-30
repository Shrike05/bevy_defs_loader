use crate::*;
use bevy::{asset::LoadedFolder, prelude::*};

pub fn setup<T: Def>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    path: Res<DefsFolder<T>>,
    mut loading_registry: ResMut<LoadingRegistry>,
) {
    let asset_handle = asset_server.load_folder(path.0);
    loading_registry.new_type();
    commands.insert_resource(DefsHandle::<T>::new(asset_handle));
}

pub fn check_and_init_lock<T: Def>(
    folder_handle: Res<DefsHandle<T>>,
    folders: Res<Assets<LoadedFolder>>,
    game_defs: Res<Assets<T>>,
    mut defs: ResMut<LoadedDefs<T>>,
    mut loading_registry: ResMut<LoadingRegistry>,
    mut def_load_state: ResMut<NextState<DefLoadState<T>>>,
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
        loading_registry.completed_type();
        def_load_state.set(DefLoadState::<T>::Ready);
    }
}

pub fn check_load_state(
    loading_reg: Res<LoadingRegistry>,
    mut load_state: ResMut<NextState<LoadState>>,
) {
    if loading_reg.is_ready() {
        load_state.set(LoadState::Ready);
    }
}
