use bevy::{app::App, prelude::*};

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

    //app.add_systems(Startup, print_config);
    //app.add_systems(Startup, (add_config, print_config.after(add_config)));

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
