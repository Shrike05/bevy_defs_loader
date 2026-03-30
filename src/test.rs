use crate::{plugin::DefsPlugin, *};
use bevy::{prelude::*, state::app::StatesPlugin};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameDef {
    hello: String,
}

impl Def for GameDef {}

fn setup_app() -> App {
    let mut app = App::new();

    app.add_plugins(StatesPlugin);
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(DefsPlugin);
    app.add_plugins(DefPlugin::<GameDef>::new("test_assets"));

    app.finish();
    app.cleanup();
    app
}

fn assert_state<T: States>(app: &App, test_state: T) {
    let def_state = app
        .world()
        .get_resource::<State<T>>()
        .expect("Def Load State does not exist");
    assert_eq!(**def_state, test_state);
}

#[test]
fn test_loading_def() {
    let mut app = setup_app();

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
    let mut app = setup_app();

    assert_state(&app, DefLoadState::<GameDef>::Loading(PhantomData));
    assert_state(&app, DefsLoadState::Loading);

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

    assert_state(&app, DefLoadState::<GameDef>::Ready);
    assert_state(&app, DefsLoadState::Ready);
}
