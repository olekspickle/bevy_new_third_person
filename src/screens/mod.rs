//! The game's main screen states and transitions between them.

use bevy::prelude::*;

mod gameover;
mod gameplay;
pub mod loading;
pub mod settings;
mod splash;
mod title;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
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
    #[default]
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Gameplay,
    // Here the menu is drawn and waiting for player interaction
    Title,
    GameOver,
}
