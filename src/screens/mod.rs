//! The game's main screen states and transitions between them.

use bevy::prelude::*;

mod gameover;
mod gameplay;
pub mod loading;
mod menu;
pub mod settings;
//mod splash;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        menu::plugin,
        //splash::plugin,
        loading::plugin,
        settings::plugin,
        gameplay::plugin,
        gameover::plugin,
    ));
}

/// The game's main screen states.
/// See https://bevy-cheatbook.github.io/programming/states.html
/// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Screen {
    // Bevy tribute <3
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // Settings screen
    Settings,
    GameOver,
}
