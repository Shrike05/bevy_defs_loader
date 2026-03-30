use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use std::marker::PhantomData;

pub trait Def: Asset + serde::de::DeserializeOwned + Clone + TypePath {}

#[derive(Resource)]
pub struct LoadedDefs<T: Def>(pub Vec<T>);

impl<T: Def> Default for LoadedDefs<T> {
    fn default() -> Self {
        LoadedDefs(vec![])
    }
}

#[derive(Resource)]
pub struct DefsFolder<T: Def>(pub &'static str, pub PhantomData<T>);

#[derive(Resource)]
pub struct DefsHandle<T: Def>(pub Handle<LoadedFolder>, pub PhantomData<T>);
