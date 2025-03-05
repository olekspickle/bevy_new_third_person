use crate::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

// This plugin listens for keyboard input and converts the input into Actions.
// Actions can then be used as a resource in other systems to act on the player input.
pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player_input_map)
        .add_systems(Update, settings.run_if(in_state(Screen::Playing)));
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    // Movement
    Forward,
    Backward,
    Left,
    Right,

    // Miscellaneous
    Mute,
    Pause,
}

fn spawn_player_input_map(mut commands: Commands) {
    let mut input_map = InputMap::default();

    input_map.insert(Action::Left, KeyCode::KeyA);
    input_map.insert(Action::Right, KeyCode::KeyD);
    input_map.insert(Action::Forward, KeyCode::KeyW);
    input_map.insert(Action::Backward, KeyCode::KeyS);

    input_map.insert(Action::Left, KeyCode::ArrowLeft);
    input_map.insert(Action::Right, KeyCode::ArrowRight);
    input_map.insert(Action::Forward, KeyCode::ArrowUp);
    input_map.insert(Action::Backward, KeyCode::ArrowDown);

    input_map.insert(Action::Pause, KeyCode::KeyP);
    input_map.insert(Action::Mute, KeyCode::KeyM);
    commands.spawn(InputManagerBundle::with_map(input_map));
}

#[derive(Resource, Default)]
pub struct Settings {
    muted: bool,
    paused: bool,
}

pub fn settings(mut settings: ResMut<Settings>, action: Query<&ActionState<Action>>) {
    let state = action.single();

    if state.just_pressed(&Action::Pause) {
        settings.paused = !settings.paused;
    }
    if state.just_pressed(&Action::Mute) {
        settings.muted = !settings.muted;
    }
}
