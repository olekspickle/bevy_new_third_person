//! The game's main screen states and transitions between them.

use crate::prelude::*;
use bevy::prelude::*;

mod credits;
mod gameover;
pub mod gameplay;
mod loading;
mod settings;
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
        credits::plugin,
        gameplay::plugin,
        gameover::plugin,
    ))
    .add_systems(Update, track_last_screen.run_if(state_changed::<Screen>));
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
    Credits,
    Settings,
    // Here the menu is drawn and waiting for player interaction
    Title,
    // During this State the actual game logic is executed
    Gameplay,
    GameOver,
}

fn track_last_screen(current: Res<State<Screen>>, mut settings: ResMut<Settings>) {
    settings.last_screen = current.get().clone();
}

fn to_gameplay_or_loading(
    _: Trigger<OnPress>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

pub fn to_title(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
pub fn to_credits(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}
pub fn to_settings(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Settings);
}
