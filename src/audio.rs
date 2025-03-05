use crate::{player, prelude::*};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::*;

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.load_resource::<AudioSources>();
    app.insert_resource(AudioInstances::default())
        .add_plugins(AudioPlugin)
        .add_systems(OnEnter(Screen::Playing), start_or_resume_audio)
        .add_systems(OnExit(Screen::Playing), pause_audio)
        .add_systems(
            Update,
            movement_sound
                .after(player::movement)
                .run_if(in_state(Screen::Playing)),
        );
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct AudioSources {
    #[dependency]
    bg_audio: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct AudioInstances {
    bg_audio: Option<Handle<AudioInstance>>,
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bg_audio: assets.load("audio/time-for-fun.ogg"),
        }
    }
}

fn start_or_resume_audio(
    global_audio: Res<Audio>,
    sources: ResMut<AudioSources>,
    mut instances: ResMut<AudioInstances>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    //global_audio.resume();

    // If there is an instance pause it
    if let Some(instance) = &instances.bg_audio {
        if let Some(instance) = audio_instances.get_mut(instance) {
            let state = instance.state();
            if let PlaybackState::Playing { .. } = state {
                instance.pause(AudioTween::default());
            }
        }
    } else {
        let bg_source = *[&sources.bg_audio].choose(&mut thread_rng()).unwrap();
        let handle = global_audio
            .play(bg_source.clone())
            .looped()
            .with_volume(0.1)
            .handle();
        instances.bg_audio = Some(handle);
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

fn movement_sound(
    global_audio: Res<Audio>,
    sources: ResMut<AudioSources>,
    mut instances: ResMut<AudioInstances>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    action: Query<&ActionState<Action>>,
) {
    let instance = instances.bg_audio.clone();

    // TODO: add actual step audio
    if let Some(instance) = &instance {
        if let Some(instance) = audio_instances.get_mut(instance) {
            match instance.state() {
                PlaybackState::Stopped | PlaybackState::Paused { .. } => {
                    let state = action.single();
                    if state.pressed(&Action::Forward)
                        | state.pressed(&Action::Backward)
                        | state.pressed(&Action::Left)
                        | state.pressed(&Action::Right)
                    {
                        let handle = global_audio
                            .play(sources.bg_audio.clone())
                            .with_volume(0.1)
                            .handle();
                        instances.bg_audio = Some(handle);
                    }
                }
                //PlaybackState::Playing { .. } => {
                //instance.pause(AudioTween::default());
                //if actions.player_movement.is_none() {
                //}
                //}
                _ => {}
            }
        }
    }
}
