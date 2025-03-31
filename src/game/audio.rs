use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::*;

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));

    app.add_plugins(AudioPlugin)
        .add_systems(OnEnter(Screen::Gameplay), start_or_resume_audio)
        .add_systems(OnExit(Screen::Gameplay), pause_audio)
        .add_systems(
            Update,
            (
                trigger_interaction_sound_effect,
                pause_audio,
                movement_sound,
            )
                .run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        );
}

/// struct of handles for long standing sounds for pause/unpase
/// or going to menu and resuming the game
#[derive(Resource, Default)]
pub struct AudioInstances {
    pub bg_music: Option<Handle<AudioInstance>>,
}

fn start_or_resume_audio(
    audio: Res<Audio>,
    sources: ResMut<AudioSources>,
    mut instances: ResMut<AudioInstances>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    audio.resume();

    // If there is an instance pause it
    if let Some(instance) = &instances.bg_music {
        if let Some(instance) = audio_instances.get_mut(instance) {
            let state = instance.state();
            if let PlaybackState::Playing { .. } = state {
                instance.pause(AudioTween::default());
            }
        }
    } else {
        let bg_source = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
        let handle = audio
            .play(bg_source.clone())
            .looped()
            .with_volume(0.1)
            .handle();
        instances.bg_music = Some(handle);
    }
}

fn pause_audio(action: Query<&ActionState<Action>>, global_audio: Res<Audio>) {
    let state = action.single();
    if state.just_pressed(&Action::Pause) {
        global_audio.pause();
    }

    //if let Some(instance) = audio_instances.get_mut(&audio.0) {
    //    instance.pause(AudioTween::default());
    //}
}

fn trigger_interaction_sound_effect(
    audio: Res<Audio>,
    audio_sources: Res<AudioSources>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => audio_sources.btn_hover.clone(),
            Interaction::Pressed => audio_sources.btn_press.clone(),
            _ => continue,
        };
        audio.play(source.clone()).with_volume(0.2);
    }
}

#[derive(Resource)]
struct StepTimer(Timer);

fn movement_sound(
    audio: Res<Audio>,
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) {
    let state = action.single();
    if state.pressed(&Action::Forward)
        | state.pressed(&Action::Backward)
        | state.pressed(&Action::Left)
        | state.pressed(&Action::Right)
        && timer.0.tick(time.delta()).just_finished()
    {
        let mut rng = thread_rng();
        let i = rng.gen_range(0..sources.steps.len());
        audio.play(sources.steps[i].clone()).with_volume(0.5);
    }
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use crate::prelude::*;
///
/// fn set_music_volume(mut sink_query: Query<&mut AudioSink, With<Music>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct Music;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::prelude::*;
/// use crate::prelude::*;
///
/// fn set_sound_effect_volume(sink_query: Query<&mut AudioSink, With<SoundEffect>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct SoundEffect;
