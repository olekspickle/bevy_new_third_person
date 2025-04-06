//#![allow(clippy::type_complexity)]
use bevy::{app::App, asset::AssetMetaCheck, log, prelude::*};

mod asset_tracking;
mod assets;
mod game;
mod screens;
mod ui;
mod utils;

pub mod prelude {
    use super::*;

    pub use asset_tracking::{LoadResource, ResourceHandles};
    pub use assets::{AudioSources, Config, Models, Textures};
    pub use game::{
        Score,
        audio::{Music, Sound, SoundEffect},
        camera::SceneCamera,
        player::Player,
        scene::uv_debug_texture,
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
    let window = WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Third Person".to_string(), // ToDo
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
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
    app.add_plugins((
        asset_tracking::plugin,
        game::plugin,
        ui::plugin,
        screens::plugin,
        assets::plugin,
    ));
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
