use bevy::{app::App, prelude::*};
use serde::{Deserialize, Serialize};

mod audio;
mod camera;
mod player;
mod scene;
mod screens;

pub use screens::{
    Screen, loading,
    settings::{self, Action},
};

pub fn game(app: &mut App) {
    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );

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
#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct GameConfig {
    scale: f32,
}

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
