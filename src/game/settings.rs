use crate::{
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
use bevy::{audio::Volume, prelude::*, ui::Display as NodeDisplay};
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player_input_map)
        .add_systems(
            OnEnter(Screen::Gameplay),
            inject_settings_from_cfg.run_if(resource_exists::<Config>),
        );
}

#[derive(Resource)]
pub struct Settings {
    pub fov: f32,
    pub sound: Sound,

    // game state things
    pub diagnostics: bool,
    pub menu_open: bool,
    pub muted: bool,
    pub paused: bool,
    pub sun_cycle: SunCycle,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sun_cycle: SunCycle::DayNight,
            sound: Sound::default(),
            diagnostics: true,
            menu_open: false,
            paused: false,
            muted: false,
            fov: 45.0, // bevy default
        }
    }
}

fn inject_settings_from_cfg(mut commands: Commands, cfg: Res<Config>) {
    commands.insert_resource(Settings {
        sound: cfg.sound.clone(),
        fov: cfg.player.fov,
        diagnostics: true,
        ..Default::default()
    });
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
    DevTools,
    Menu,
    Mute,
    Pause,
    ToggleDiagnostics,
    Toot,
    FovIncr,
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
    input_map.insert(Action::Menu, KeyCode::Escape);
    input_map.insert(Action::ToggleDiagnostics, KeyCode::KeyF);

    input_map.insert(Action::DevTools, KeyCode::Backquote);
    input_map.insert(Action::Toot, KeyCode::KeyC);
    input_map.insert(Action::FovIncr, KeyCode::KeyV);

    commands.spawn(input_map);
}
