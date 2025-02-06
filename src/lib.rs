#![allow(clippy::type_complexity)]

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

mod actions;
mod audio;
mod camera;
mod loading;
mod menu;
mod player;
mod scene;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum Screen {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // TODO: cinematics
    Interlude,
}

pub fn game(app: &mut App) {
    app.init_state::<Screen>().add_plugins((
        menu::plugin,
        scene::plugin,
        player::plugin,
        camera::plugin,
        loading::plugin,
        actions::plugin,
        audio::plugin,
        ThirdPersonCameraPlugin,
    ));

    #[cfg(debug_assertions)]
    {
        app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
    }
}
