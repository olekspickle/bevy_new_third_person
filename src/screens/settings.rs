use crate::prelude::*;
use bevy::{prelude::*, ui::Display as NodeDisplay};
use bevy_kira_audio::*;
use leafwing_input_manager::prelude::*;

use super::gameplay::{MuteMarker, PauseMarker};

// This plugin listens for keyboard input and converts the input into Actions.
// Actions can then be used as a resource in other systems to act on the player input.
pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player_input_map)
        .add_systems(Update, toggle_global);
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    // Movement
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,

    // Miscellaneous
    Mute,
    Pause,
    Settings,
    ToggleDiagnostics,
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

    input_map.insert(Action::Jump, KeyCode::Space);
    input_map.insert(Action::Crouch, KeyCode::ControlLeft);

    input_map.insert(Action::Pause, KeyCode::KeyP);
    input_map.insert(Action::Mute, KeyCode::KeyM);
    input_map.insert(Action::Settings, KeyCode::Escape);
    input_map.insert(Action::ToggleDiagnostics, KeyCode::KeyF);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

#[derive(Resource, Default)]
pub struct Sound {
    general: f64,
    music: f64,
    sfx: f64,
}

impl Sound {
    const DEFAULT: Self = Sound {
        general: 0.5,
        music: 0.5,
        sfx: 0.7,
    };
    //fn splat(level: f64) -> Self {
    //    Self {
    //        general: level,
    //        music: level,
    //        sfx: level,
    //    }
    //}
}

#[derive(Resource)]
pub struct Settings {
    muted: bool,
    paused: bool,
    sound: Sound,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound: Sound::DEFAULT,
            paused: false,
            muted: false,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn toggle_global(
    mut commands: Commands,
    global_audio: Res<Audio>,
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    action: Query<&ActionState<Action>>,
    muted: Query<Entity, With<MuteMarker>>,
    paused: Query<Entity, With<PauseMarker>>,
    mut perf_ui: Query<&mut Node, With<PerfUiMarker>>,
) {
    let state = action.single();

    if state.just_pressed(&Action::ToggleDiagnostics) {
        let mut perf_ui_node = perf_ui.single_mut();
        if perf_ui_node.display == NodeDisplay::None {
            perf_ui_node.display = NodeDisplay::Flex;
        } else {
            perf_ui_node.display = NodeDisplay::None;
        }
    }

    if state.just_pressed(&Action::Pause) {
        if time.is_paused() || settings.paused {
            if let Ok(e) = paused.get_single() {
                commands
                    .entity(e)
                    .spawn((TextColor(LABEL), BackgroundColor(NODE_BG)));
            }
            global_audio.resume();
            time.unpause();
        } else {
            if let Ok(e) = paused.get_single() {
                commands
                    .entity(e)
                    .spawn((TextColor(NODE_BG), BackgroundColor(LABEL)));
            }
            global_audio.pause();
            time.pause();
        }
        settings.paused = !settings.paused;
    }

    if state.just_pressed(&Action::Mute) {
        if settings.muted {
            global_audio.set_volume(settings.sound.general);

            if let Ok(e) = muted.get_single() {
                commands
                    .entity(e)
                    .spawn((TextColor(LABEL), BackgroundColor(NODE_BG)));
            }
        } else {
            if let Ok(e) = muted.get_single() {
                commands
                    .entity(e)
                    .spawn((TextColor(NODE_BG), BackgroundColor(LABEL)));
            }
            global_audio.set_volume(0.0);
        }
        settings.muted = !settings.muted;
    }
}
