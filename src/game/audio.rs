use bevy::prelude::*;

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

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use bevy_seedling::prelude::*;
/// use crate::prelude::*;
///
/// fn set_music_volume(mut query: Query<&mut AudioSink, With<Music>>) {
///     for mut sink in &mut query {
///         sink.set_volume(0.5);
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
/// fn set_sfx_volume(mut query: Query<&mut AudioSink, With<SoundEffect>>) {
///     for mut sink in &mut query {
///         sink.set_volume(0.5);
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct SoundEffect;
