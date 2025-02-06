use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    actions::{self, Actions},
    loading::AudioAssets,
    Screen,
};

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_systems(OnEnter(Screen::Playing), start_or_resume_audio)
        .add_systems(OnExit(Screen::Playing), pause_audio)
        .add_systems(
            Update,
            control_flying_sound
                .after(actions::set_movement)
                .run_if(in_state(Screen::Playing)),
        );
}

#[derive(Resource)]
struct MainTheme(Handle<AudioInstance>);

fn start_or_resume_audio(
    mut commands: Commands,
    bg_audio: Res<MainTheme>,
    global_audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    //global_audio.resume();
    if let Some(instance) = audio_instances.get_mut(&bg_audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } | PlaybackState::Pausing { .. } => {
                instance.resume(AudioTween::default());
            }
            _ => {
                let handle = global_audio
                    .play(audio_assets.bg_play.clone())
                    .looped()
                    .with_volume(0.3)
                    .handle();
                commands.insert_resource(MainTheme(handle));
            }
        }
    }
}

fn pause_audio(
    //global_audio: Res<Audio>,
    audio: Res<MainTheme>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    //global_audio.pause();
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        instance.pause(AudioTween::default());
    }
}

fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<MainTheme>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.player_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.player_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
