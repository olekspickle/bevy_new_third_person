use bevy::{audio::Volume, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Sound {
    pub general: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            general: 1.0,
            music: 0.5,
            sfx: 0.5,
        }
    }
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
        Music,
        AudioPlayer(handle),
        PlaybackSettings {
            volume: Volume::Linear(volume),
            ..PlaybackSettings::LOOP
        },
    )
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sfx(handle: Handle<AudioSource>, volume: f32) -> impl Bundle {
    (
        SoundEffect,
        AudioPlayer(handle),
        PlaybackSettings {
            volume: Volume::Linear(volume),
            ..PlaybackSettings::DESPAWN
        },
    )
}
