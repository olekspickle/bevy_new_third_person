use crate::prelude::*;
use bevy::{prelude::*, ui::Display as NodeDisplay};
use bevy_kira_audio::*;
use leafwing_input_manager::prelude::*;

use super::gameplay::{MuteLabel, PauseLabel};

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
    pub general: f64,
    pub music: f64,
    pub sfx: f64,
}

impl Sound {
    const DEFAULT: Self = Sound {
        general: 0.5,
        music: 0.1,
        sfx: 0.3,
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
    pub muted: bool,
    pub paused: bool,
    pub sound: Sound,
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

#[allow(clippy::type_complexity)]
fn toggle_global(
    global_audio: Res<Audio>,
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    action: Query<&ActionState<Action>>,
    mut set: ParamSet<(
        Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
        Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    )>,
    mut perf_ui: Query<&mut Node, With<PerfUiMarker>>,
) {
    let state = action.single();

    if state.just_pressed(&Action::ToggleDiagnostics) {
        if let Ok(mut perf_ui) = perf_ui.get_single_mut() {
            if perf_ui.display == NodeDisplay::None {
                perf_ui.display = NodeDisplay::Flex;
            } else {
                perf_ui.display = NodeDisplay::None;
            }
        }
    }

    if state.just_pressed(&Action::Pause) {
        if let Ok((mut bg, mut color)) = set.p0().get_single_mut() {
            if time.is_paused() || settings.paused {
                global_audio.resume();
                time.unpause();
                *color = TextColor(LABEL);
                *bg = BackgroundColor(NODE_BG);
            } else {
                global_audio.pause();
                time.pause();
                *color = TextColor(NODE_BG);
                *bg = BackgroundColor(LABEL);
            }
        }
        settings.paused = !settings.paused;
    }

    if state.just_pressed(&Action::Mute) {
        if let Ok((mut bg, mut color)) = set.p1().get_single_mut() {
            if settings.muted {
                global_audio.set_volume(settings.sound.general);
                *color = TextColor(LABEL);
                *bg = BackgroundColor(NODE_BG);
            } else {
                global_audio.set_volume(0.0);
                *color = TextColor(NODE_BG);
                *bg = BackgroundColor(LABEL);
            }
        }
        settings.muted = !settings.muted;
    }
}
