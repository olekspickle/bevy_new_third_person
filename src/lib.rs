#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod camera;
mod loading;
mod menu;
mod player;
mod scene;

use crate::{
    actions::ActionsPlugin, audio::InternalAudioPlugin, camera::CameraPlugin,
    loading::LoadingPlugin, menu::MenuPlugin, player::PlayerPlugin, scene::ScenePlugin,
};

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            MenuPlugin,
            ScenePlugin,
            PlayerPlugin,
            CameraPlugin,
            LoadingPlugin,
            //ActionsPlugin,
            InternalAudioPlugin,
            ThirdPersonCameraPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
