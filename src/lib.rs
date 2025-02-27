use bevy::{app::App, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

mod audio;
mod camera;
mod player;
mod scene;
mod screens;
mod utils;

pub use camera::{SceneCamera, Ui};
pub use screens::{
    Screen, loading,
    settings::{Action, Settings},
};
pub use utils::despawn;

pub fn game(app: &mut App) {
    //app.configure_sets(
    //    Update,
    //    (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    //);
    app.add_plugins((RonAssetPlugin::<Config>::new(&["config.ron"]),));
    app.add_systems(Startup, (add_config, print_config.after(add_config)));

    app.add_plugins((
        scene::plugin,
        player::plugin,
        camera::plugin,
        screens::plugin,
        audio::plugin,
    ));

    //#[cfg(debug_assertions)]
    //use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //#[cfg(debug_assertions)]
    //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
}
fn add_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cfg = ConfigHandle(asset_server.load("config.ron"));
    commands.insert_resource(cfg);
}
fn print_config(cfg: Res<Config>, cfg_handle: Res<ConfigHandle>) {
    //println!("scale: {}", cfg.scale);
    //println!("scale: {}", cfg_handle.0.clone());
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    scale: f32,
}

#[derive(Resource)]
struct ConfigHandle(Handle<Config>);

#[derive(Default, Resource)]
pub struct Score(pub i32);

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
