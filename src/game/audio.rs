use bevy::prelude::*;
use bevy_seedling::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default());
}

#[derive(Resource, Default)]
pub struct Sound {
    pub general: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Sound {
    pub const DEFAULT: Self = Sound {
        general: 0.3,
        music: 0.1,
        sfx: 0.3,
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
/// fn set_music_volume(mut query: Query<&mut PlaybackSettings, With<Music>>) {
///     for mut playback in &mut query {
///         playback.set_volume(Volume::Linear(0.5));
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
/// fn set_sfx_volume(mut query: Query<&mut PlaybackSettings, With<SoundEffect>>) {
///     for mut playback in &mut query {
///         playback.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct SoundEffect;
