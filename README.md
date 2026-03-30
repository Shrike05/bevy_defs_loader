# Usage
## Initialization
```rs
let mut app = App::new()

app.add_plugins(DefaultPlugins);

app.add_plugins(DefsPlugin);
```

## Initialize Custom Def

* Create the definition struct
```rs
#[derive(Asset, TypePath, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct GameDef {
    hello: String,
}
```
note: use of serde for deserialization.

* Add the DefPlugin<T> where T is your custom struct. Do this to your main app
```rs
app.add_plugins(DefPlugin::<GameDef>::new("test_assets"));
```

## How to access Defs
Once your custom defs have been loaded you may use them like so
```rs
fn my_system(defs: Res<LoadedDefs<GameDef>>){
    println!("{}", defs[0].hello);
}
```

## Load States
The library exports two load states:

This one for the global LoadState of all initialized Defs in the game, Recommended for most situations
```rs
#[derive(States, Clone, Copy, Debug, Hash, PartialEq, Eq, Default)]
pub enum DefsLoadState {
    #[default]
    Loading,
    Ready,
}
```

For induvidual Defs, in case you want to pick and choose which systems will run given X Defs have been loaded
```rs
#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DefLoadState<T: Def> {
    Loading(PhantomData<T>),
    Ready,
}
```
These are initalized with the plugins so you simply need to use them for scheduling systems
