use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Def: Asset + serde::de::DeserializeOwned + Clone + TypePath + Eq + Debug + Hash {}

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

impl<T: Def> DefsHandle<T> {
    pub fn new(handle: Handle<LoadedFolder>) -> Self {
        DefsHandle(handle, PhantomData)
    }
}

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DefLoadState<T: Def> {
    Loading(PhantomData<T>),
    Ready,
}

#[derive(States, Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LoadingRegistry {
    total_types: usize,
    ready_types: usize,
}

impl LoadingRegistry {
    pub fn new_type(&mut self) {
        self.total_types += 1;
    }

    pub fn completed_type(&mut self) {
        self.ready_types += 1;
    }

    pub fn is_ready(&self) -> bool {
        self.total_types == self.ready_types
    }
}
