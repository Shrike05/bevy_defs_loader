use crate::loader::*;
use crate::*;
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use std::marker::PhantomData;

pub struct DefsPlugin<T: Def> {
    def_folder: &'static str,
    pub _marker: PhantomData<T>,
}

impl<T: Def> Plugin for DefsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(DefsFolder::<T>(self.def_folder, PhantomData));
        app.insert_resource(LoadedDefs::<T>::default());
        app.init_asset::<T>();
        app.add_plugins(TomlAssetPlugin::<T>::new(&["toml"]));
        app.add_systems(Startup, setup::<T>);
        app.add_systems(Update, check_and_init_lock::<T>);
    }
}

impl<T: Def> DefsPlugin<T> {
    pub fn new(def_folder: &'static str) -> Self {
        DefsPlugin {
            def_folder,
            _marker: PhantomData,
        }
    }
}
