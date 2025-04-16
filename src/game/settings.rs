use crate::{
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
use bevy::{prelude::*, ui::Display as NodeDisplay};
use leafwing_input_manager::prelude::*;

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
    Dash,
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
    input_map.insert(Action::Dash, KeyCode::ShiftLeft);

    input_map.insert(Action::Pause, KeyCode::KeyP);
    input_map.insert(Action::Mute, KeyCode::KeyM);
    input_map.insert(Action::Settings, KeyCode::Escape);
    input_map.insert(Action::ToggleDiagnostics, KeyCode::KeyF);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

#[derive(Resource)]
pub struct Settings {
    pub fov: f32,
    pub muted: bool,
    pub paused: bool,
    pub diagnostics: bool,
    pub sound: Sound,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sound: Sound::DEFAULT,
            diagnostics: false,
            paused: false,
            muted: false,
            fov: 90.0,
        }
    }
}

#[allow(clippy::type_complexity)]
fn toggle_global(
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    action: Query<&ActionState<Action>>,
    mut label_set: ParamSet<(
        Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
        Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    )>,

    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
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
            settings.diagnostics = !settings.diagnostics;
        }
    }

    if state.just_pressed(&Action::Pause) || state.just_pressed(&Action::Settings) {
        if let Ok((mut bg, mut color)) = label_set.p0().get_single_mut() {
            if time.is_paused() || settings.paused {
                time.unpause();
                *color = TextColor(LABEL);
                *bg = BackgroundColor(NODE_BG);
            } else {
                time.pause();
                *color = TextColor(NODE_BG);
                *bg = BackgroundColor(LABEL);
            }
        }
        // TODO: use seedling under feature
        for s in &mut music.iter_mut().chain(sfx.iter_mut()) {
            s.toggle();
        }
        settings.paused = !settings.paused;
    }

    if state.just_pressed(&Action::Mute) {
        if let Ok((mut bg, mut color)) = label_set.p1().get_single_mut() {
            if settings.muted {
                // TODO: use seedling under feature
                for s in &mut music {
                    s.set_volume(settings.sound.general * settings.sound.music);
                }
                for s in &mut sfx {
                    s.set_volume(settings.sound.general * settings.sound.sfx);
                }
                *color = TextColor(LABEL);
                *bg = BackgroundColor(TRANSPARENT);
            } else {
                // TODO: use seedling under feature
                for s in &mut music.iter_mut().chain(sfx.iter_mut()) {
                    s.set_volume(0.0);
                }
                *color = TextColor(NODE_BG);
                *bg = BackgroundColor(LABEL);
            }
        }
        settings.muted = !settings.muted;
    }
}
