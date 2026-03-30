use crate::*;
use bevy::{prelude::*, state::app::StatesPlugin};
use serde::Deserialize;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct GameDef {
    hello: String,
}

impl Def for GameDef {}

#[cfg(test)]
fn test_loading_def() {
    let mut app = App::new();

    app.add_plugins(StatesPlugin);
    app.add_plugins(MinimalPlugins);
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(DefsPlugin::<GameDef>::new("test_assets"));

    app.finish();
    app.cleanup();

    loop {
        app.update();
        let gamedefs = app.world().get_resource::<LoadedDefs<GameDef>>().unwrap();
        if !gamedefs.0.is_empty() {
            assert_eq!(gamedefs.0.len(), 1);
            break;
        }
    }
}
