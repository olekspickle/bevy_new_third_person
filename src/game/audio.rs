use bevy::{audio::Volume, prelude::*};

#[derive(Resource, Default)]
pub struct Sound {
    pub general: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Sound {
    pub const DEFAULT: Self = Sound {
        general: 0.5,
        music: 0.5,
        sfx: 0.5,
    };
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>, volume: f32) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings {
            volume: Volume::Linear(volume),
            ..PlaybackSettings::LOOP
        },
        Music,
    )
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}
