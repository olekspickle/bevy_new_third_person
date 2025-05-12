//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
use bevy::{
    audio::Volume,
    dev_tools::states::log_transitions,
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
    app.add_systems(Update, toggle_debug_ui);
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
    mut label_set: ParamSet<(
        Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
        Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    )>,

    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) -> Result {
    let state = action.single()?;

    if state.just_pressed(&Action::Pause) || state.just_pressed(&Action::Menu) {
        if let Ok((mut bg, mut color)) = label_set.p0().single_mut() {
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
            s.pause();
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
                // TODO: use seedling when it's migrated to 0.16
                for mut s in music {
                    s.set_volume(Volume::Linear(
                        settings.sound.general * settings.sound.music,
                    ));
                }
                for mut s in sfx {
                    s.set_volume(Volume::Linear(settings.sound.general * settings.sound.sfx));
                }
                *color = TextColor(WHITEISH);
                *bg = BackgroundColor(TRANSPARENT);
            } else {
                // TODO: use seedling when it's migrated to 0.16
                for mut s in music.iter_mut().chain(sfx.iter_mut()) {
                    s.mute();
                }
                *color = TextColor(GRAY);
                *bg = BackgroundColor(WHITEISH);
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
