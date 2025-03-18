use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use rand::prelude::*;

// This plugin is responsible to control the game audio
pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin)
        .add_systems(OnEnter(Screen::Gameplay), start_or_resume_audio)
        .add_systems(OnExit(Screen::Gameplay), pause_audio)
        .add_systems(
            Update,
            (trigger_interaction_sound_effect, movement_sound)
                .run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct AudioSources {
    // SFX
    #[dependency]
    pub btn_hover: Handle<AudioSource>,
    #[dependency]
    pub btn_press: Handle<AudioSource>,
    #[dependency]
    pub walk: Handle<AudioSource>,

    // music
    #[dependency]
    pub bg_music: Handle<AudioSource>,
}

/// struct of handles for long standing sounds for pause/unpase
/// or going to menu and resuming the game
#[derive(Resource, Default)]
pub struct AudioInstances {
    pub bg_music: Option<Handle<AudioInstance>>,
}

impl AudioSources {
    pub const WALK: &'static str = "audio/sfx/walk.ogg";
    pub const BTN_HOVER: &'static str = "audio/sfx/btn-hover.ogg";
    pub const BTN_PRESS: &'static str = "audio/sfx/btn-press.ogg";

    pub const BG_MUSIC: &'static str = "audio/music/time-for-fun.ogg";
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            walk: assets.load(Self::WALK),
            btn_hover: assets.load(Self::BTN_HOVER),
            btn_press: assets.load(Self::BTN_PRESS),
            bg_music: assets.load(Self::BG_MUSIC),
        }
    }
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

fn movement_sound(
    audio: Res<Audio>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) {
    // TODO: add actual step audio
    let state = action.single();
    if state.pressed(&Action::Forward)
        | state.pressed(&Action::Backward)
        | state.pressed(&Action::Left)
        | state.pressed(&Action::Right)
    {
        audio.play(sources.walk.clone()).with_volume(0.5);
    }
}

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::prelude::*;
/// use boat_game::audio::Music;
///
/// fn set_music_volume(sink_query: Query<&AudioSink, With<Music>>) {
///     for sink in &sink_query {
///         sink.set_volume(0.5);
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct Music;

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::prelude::*;
/// use boat_game::audio::SoundEffect;
///
/// fn set_sound_effect_volume(sink_query: Query<&AudioSink, With<SoundEffect>>) {
///     for sink in &sink_query {
///         sink.set_volume(0.5);
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct SoundEffect;
