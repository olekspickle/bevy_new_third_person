//#![allow(clippy::type_complexity)]
use bevy::{app::App, asset::AssetMetaCheck, log, prelude::*, window::WindowResolution};

mod audio;
mod dev_tools;
mod game;
mod loading;
mod pre_load;
mod screens;
mod ui;
mod utils;

pub(crate) mod prelude {
    use super::*;

    pub use audio::{Music, Sound, SoundEffect, music, sfx};
    pub use game::{
        Score,
        camera::SceneCamera,
        player::{JumpTimer, Player, StepTimer},
        scene,
        settings::{Action, Settings},
        skybox::SunCycle,
    };
    pub use loading::{AudioSources, LoadResource, Models, ResourceHandles, Textures};
    pub use pre_load::{Config, Credits};
    pub use screens::Screen;
    pub use ui::*;
    pub use utils::despawn;
}

pub fn game(app: &mut App) {
    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );
    let mut resolution = WindowResolution::default();
    resolution.set_physical_resolution(1600, 1000);
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Game".to_string(),
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            resolution,
            ..default()
        }),
        ..default()
    };
    let assets = AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics on web build on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    };
    let log_level = log::LogPlugin {
        level: log::Level::TRACE,
        filter: "info,wgpu=warn".to_string(),
        ..Default::default()
    };

    app.add_plugins(DefaultPlugins.set(window).set(assets).set(log_level));

    // custom plugins. the order is important
    // be sure you use resources/types AFTER you add plugins that insert them
    app.add_plugins((loading::plugin, game::plugin, ui::plugin, screens::plugin));

    #[cfg(feature = "dev_native")]
    app.add_plugins(dev_tools::plugin);
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
