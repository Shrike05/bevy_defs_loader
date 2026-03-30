use crate::{plugin::DefsPlugin, *};
use bevy::{prelude::*, state::app::StatesPlugin};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameDef {
    hello: String,
}

impl Def for GameDef {}

#[test]
fn test_loading_def() {
    let mut app = App::new();

    app.add_plugins(StatesPlugin);
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(DefsPlugin);
    app.add_plugins(DefPlugin::<GameDef>::new("test_assets"));

    app.finish();
    app.cleanup();

    loop {
        app.update();

        match app.world().get_resource::<LoadedDefs<GameDef>>() {
            Some(gamedefs) => {
                if !gamedefs.0.is_empty() {
                    assert_eq!(gamedefs.0.len(), 1);
                    break;
                }
            }
            None => {
                panic!("Couldn't Get LoadedDefs");
            }
        }
    }
}

#[test]
fn test_states_def() {
    use std::marker::PhantomData;

    let mut app = App::new();

    app.add_plugins(StatesPlugin);
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(DefsPlugin);
    app.add_plugins(DefPlugin::<GameDef>::new("test_assets"));

    app.finish();
    app.cleanup();

    let def_state = app
        .world()
        .get_resource::<State<DefLoadState<GameDef>>>()
        .expect("Def Load State does not exist");
    assert_eq!(**def_state, DefLoadState::<GameDef>::Loading(PhantomData));

    let load_state = app
        .world()
        .get_resource::<State<LoadState>>()
        .expect("Load State does not exist");
    assert_eq!(**load_state, LoadState::Loading);

    loop {
        app.update();
        let gamedefs = app
            .world()
            .get_resource::<LoadedDefs<GameDef>>()
            .expect("LoadedDefs does not exist");
        if !gamedefs.0.is_empty() {
            assert_eq!(gamedefs.0.len(), 1);
            break;
        }
    }
    app.update();

    let def_state = app
        .world()
        .get_resource::<State<DefLoadState<GameDef>>>()
        .expect("Def Load State does not exist");
    assert_eq!(**def_state, DefLoadState::<GameDef>::Ready);

    let load_state = app
        .world()
        .get_resource::<State<LoadState>>()
        .expect("Load State does not exist");
    assert_eq!(**load_state, LoadState::Ready);
}
