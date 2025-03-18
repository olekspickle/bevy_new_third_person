#![allow(clippy::type_complexity)]
use bevy::{app::App, prelude::*};

mod asset_tracking;
mod config;
mod game;
mod screens;
mod ui;
mod utils;

pub mod prelude {
    use super::*;

    pub use asset_tracking::{LoadResource, ResourceHandles};
    pub use config::{Config, Models, Textures};
    pub use game::{
        Score,
        audio::{AudioInstances, AudioSources, Music, SoundEffect},
        camera::SceneCamera,
        skybox::Sun,
    };
    pub use screens::{
        Screen, loading,
        settings::{Action, Settings},
    };
    pub use ui::*;
    pub use utils::despawn;
}

pub fn game(app: &mut App) {
    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );

    app.add_plugins((
        asset_tracking::plugin,
        game::plugin,
        screens::plugin,
        config::plugin,
    ));

    //#[cfg(debug_assertions)]
    //use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
    //#[cfg(debug_assertions)]
    //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSet {
    TickTimers,
    RecordInput,
    Update,
}
