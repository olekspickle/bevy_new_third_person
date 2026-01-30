//! The game's main screen states and transitions between them.
use crate::*;
use bevy::ui::Val::*;

mod credits;
mod gameplay;
mod loading;
mod settings;
mod splash;
mod title;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        settings::plugin,
        credits::plugin,
        gameplay::plugin,
    ))
    .add_systems(Update, track_last_screen.run_if(state_changed::<Screen>))
    .add_observer(on_back)
    .add_observer(on_go_to);
}

/// The game's main screen states.
/// See <https://bevy-cheatbook.github.io/programming/states.html>
/// Or <https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs>
#[derive(Component, States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum Screen {
    // TODO: splash should be first
    // Bevy tribute <3
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Tutorial,
    Credits,
    Settings,
    // Here the menu is drawn and waiting for player interaction
    Title,
    // During this State the actual game logic is executed
    Gameplay,
}

// TODO: figure out how to make it a cool observer
// mut transitions: On<StateTransitionEvent<Screen>>,
fn track_last_screen(
    mut transitions: MessageReader<StateTransitionEvent<Screen>>,
    mut state: ResMut<GameState>,
) {
    let Some(transition) = transitions.read().last() else {
        return;
    };
    state.last_screen = transition.clone().exited.unwrap_or(Screen::Title);
}

fn on_back(
    trigger: On<Back>,
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    // Do not go to the title on back, we'd rather handle it in gameplay observers
    if *screen.get() == Screen::Gameplay {
        return;
    }

    let back = trigger.event();
    next_screen.set(back.screen.clone());
}

pub fn on_go_to(goto: On<GoTo>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(goto.event().0.clone());
}

pub mod to {
    use super::*;

    pub fn title(
        _: On<Pointer<Click>>,
        mut commands: Commands,
        mut state: ResMut<GameState>,
        mut modals: ResMut<Modals>,
    ) {
        state.reset();
        modals.clear();
        commands.trigger(GoTo(Screen::Title));
    }

    // pub fn go_to(on: On<Pointer<Click>>, screen_q: Query<&Screen>, mut commands: Commands) {
    //     info!("going to");
    //     if let Ok(screen) = screen_q.get(on.entity) {
    //         info!("going to {screen:?}");
    //         commands.trigger(GoTo(screen.clone()));
    //     }
    // }
    pub fn settings(_: On<Pointer<Click>>, mut commands: Commands) {
        commands.trigger(GoTo(Screen::Settings));
    }
    pub fn credits(_: On<Pointer<Click>>, mut commands: Commands) {
        commands.trigger(GoTo(Screen::Credits));
    }
    pub fn gameplay_or_loading(
        _: On<Pointer<Click>>,
        resource_handles: Res<ResourceHandles>,
        mut next_screen: ResMut<NextState<Screen>>,
    ) {
        if resource_handles.is_all_done() {
            next_screen.set(Screen::Gameplay);
        } else {
            next_screen.set(Screen::Loading);
        }
    }
}
