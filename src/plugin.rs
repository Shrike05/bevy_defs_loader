use crate::loader::*;
use crate::types::LoadingRegistry;
use crate::*;
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use std::marker::PhantomData;

pub struct DefsPlugin;

pub struct DefPlugin<T: Def> {
    def_folder: &'static str,
    pub _marker: PhantomData<T>,
}

impl<T: Def> Plugin for DefPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_state(DefLoadState::<T>::Loading(PhantomData));
        app.insert_resource(DefsFolder::<T>(self.def_folder, PhantomData));
        app.insert_resource(LoadedDefs::<T>::default());
        app.init_asset::<T>();
        app.add_plugins(TomlAssetPlugin::<T>::new(&["toml"]));
        app.add_systems(
            Startup,
            setup::<T>.run_if(in_state(DefLoadState::<T>::Loading(PhantomData))),
        );
        app.add_systems(
            Update,
            check_and_init_lock::<T>.run_if(in_state(DefLoadState::<T>::Loading(PhantomData))),
        );
    }
}

impl<T: Def> DefPlugin<T> {
    pub fn new(def_folder: &'static str) -> Self {
        DefPlugin {
            def_folder,
            _marker: PhantomData,
        }
    }
}

impl Plugin for DefsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadingRegistry::default());
        app.insert_state(DefsLoadState::Loading);
        app.add_systems(
            Update,
            check_load_state.run_if(in_state(DefsLoadState::Loading)),
        );
    }
}
