//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
use bevy::{
    audio::Volume,
    dev_tools::states::log_transitions,
    input::common_conditions::input_just_pressed,
    prelude::*,
    ui::{Display as NodeDisplay, UiDebugOptions},
};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            toggle_mute,
            toggle_pause,
            toggle_diagnostics,
            log_transitions::<Screen>,
        )
            .chain(),
    );

    #[cfg(feature = "dev_native")]
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(KeyCode::Backquote)),
    );
}

fn toggle_diagnostics(
    mut settings: ResMut<Settings>,
    action: Query<&ActionState<Action>>,
    mut perf_ui: Query<&mut Node, With<PerfUiMarker>>,
) -> Result {
    let state = action.single()?;

    if state.just_pressed(&Action::ToggleDiagnostics) {
        if let Ok(mut perf_ui) = perf_ui.single_mut() {
            if perf_ui.display == NodeDisplay::None {
                perf_ui.display = NodeDisplay::Flex;
            } else {
                perf_ui.display = NodeDisplay::None;
            }
            settings.diagnostics = !settings.diagnostics;
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
fn toggle_pause(
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    action: Query<&ActionState<Action>>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) -> Result {
    let state = action.single()?;

    if state.just_pressed(&Action::Pause) || state.just_pressed(&Action::Menu) {
        if let Ok((mut bg, mut color)) = label.single_mut() {
            if time.is_paused() || settings.paused {
                time.unpause();
                *color = TextColor(WHITEISH);
                *bg = BackgroundColor(TRANSPARENT);
            } else {
                time.pause();
                *color = TextColor(GRAY);
                *bg = BackgroundColor(WHITEISH);
            }
        }
        // TODO: use seedling when it's migrated to 0.16
        for s in music.iter_mut().chain(sfx.iter_mut()) {
            s.toggle_playback();
        }
        settings.paused = !settings.paused;
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
fn toggle_mute(
    mut settings: ResMut<Settings>,
    action: Query<&ActionState<Action>>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) -> Result {
    let state = action.single()?;

    if state.just_pressed(&Action::Mute) {
        if let Ok((mut bg, mut color)) = label.single_mut() {
            if settings.muted {
                *color = TextColor(WHITEISH);
                *bg = BackgroundColor(TRANSPARENT);
            } else {
                *color = TextColor(GRAY);
                *bg = BackgroundColor(WHITEISH);
            }
            // TODO: use seedling when it's migrated to 0.16
            // s.set_volume(Volume::Linear(settings.sound.general * settings.sound.sfx));
            for mut s in music.iter_mut().chain(sfx.iter_mut()) {
                s.toggle_mute();
            }
        }
        settings.muted = !settings.muted;
    }

    Ok(())
}

#[cfg(feature = "dev_native")]
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
