//! An abstraction for changing music of the game depending on some triggers
use super::*;
use crate::player::Player;
use avian3d::prelude::Collisions;
use bevy::time::common_conditions::on_timer;
use std::{collections::HashMap, time::Duration};

pub fn plugin(app: &mut App) {
    app.init_state::<Mood>();
    app.init_resource::<MusicPlaybacks>();

    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack)
        .add_systems(
            Update,
            trigger_mood_change
                .run_if(in_state(Screen::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))),
        )
        .add_observer(change_mood)
        .add_observer(MusicPlaybacks::track_entity)
        .add_observer(MusicPlaybacks::keep_playlist_playing);
}

#[derive(Component, States, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[reflect(Component)]
pub enum Mood {
    #[default]
    Exploration,
    Combat,
}

fn start_soundtrack(
    settings: Res<Settings>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
) {
    let mut rng = rand::rng();
    let handle = sources.explore.pick(&mut rng);

    let e = commands
        .spawn((
            MusicPool,
            SamplePlayer::new(handle.clone())
                .with_volume(settings.music())
                .looping(),
            sample_effects![VolumeNode {
                volume: Volume::SILENT,
                ..default()
            }],
            FadeIn,
        ))
        .id();
    let mp: MusicPlaybacks = [(Mood::default(), e)].into_iter().collect();
    commands.insert_resource(mp);
}

fn stop_soundtrack(
    mut music: Query<&mut PlaybackSettings, With<MusicPool>>,
    mut music_pbs: ResMut<MusicPlaybacks>,
) {
    for (_, e) in music_pbs.iter_mut() {
        let Ok(mut s) = music.get_mut(*e) else {
            continue;
        };
        s.pause();
    }
}

fn trigger_mood_change(
    collisions: Collisions,
    mood: Res<State<Mood>>,
    zones: Query<(Entity, &Mood)>,
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
) {
    let current_mood = *mood.get();
    let Ok(player) = player.single_mut() else {
        return;
    };

    for (e, &mood) in zones.iter() {
        if collisions.contains(player, e) {
            if current_mood != mood {
                debug!("Trigger changing mood {current_mood:?} -> {mood:?}");
                commands.trigger(ChangeMood {
                    mood,
                    entity: player,
                });
            }
        }
    }
}

/// Every time the current mood in GameState resource changes,
/// this system is run to trigger the song change
fn change_mood(
    on: On<ChangeMood>,
    settings: Res<Settings>,
    music_pbs: ResMut<MusicPlaybacks>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
    mut next_mood: ResMut<NextState<Mood>>,
) {
    let mut rng = rand::rng();
    for (mood, track) in music_pbs.iter() {
        if mood != &on.mood {
            commands.entity(*track).insert(FadeOut);
        }
    }
    next_mood.set(on.mood);

    if let Some(track) = music_pbs.get(&on.mood) {
        debug!("found existing track, fading IN: {track}",);
        commands.entity(*track).insert(FadeIn);
        return;
    }
    debug!("did not find existing track, spawning new {:?}", on.mood);

    // Spawn a new music with the appropriate soundtrack based on new mood
    // Volume is set to start at zero and is then increased by the fade_in system.
    let handle = match &on.mood {
        Mood::Exploration => sources.explore.pick(&mut rng),
        Mood::Combat => sources.combat.pick(&mut rng),
    };

    commands.spawn((
        MusicPool,
        SamplePlayer::new(handle.clone())
            .with_volume(settings.music())
            .looping(),
        sample_effects![VolumeNode {
            volume: Volume::SILENT,
            ..default()
        }],
        FadeIn,
        on.mood,
    ));
}

/// Map of entities that are currently playing music for a specific mood
/// Use them to keep track of [`PlaybackSettings`] and play/pause instead of spawning new ones
#[derive(Resource, Reflect, Debug, Clone, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct MusicPlaybacks(HashMap<Mood, Entity>);

impl FromIterator<(Mood, Entity)> for MusicPlaybacks {
    fn from_iter<T: IntoIterator<Item = (Mood, Entity)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl MusicPlaybacks {
    fn track_entity(
        on: On<Add, (Mood, SamplePlayer)>,
        moods: Query<&Mood>,
        mut music_pbs: ResMut<MusicPlaybacks>,
    ) {
        if let Ok(&mood) = moods.get(on.entity) {
            debug!("adding entity for {mood:?} {}", on.entity);
            music_pbs.insert(mood, on.entity);
        }
    }

    /// When [`SamplePlayer`] finishes playing, it spawnes next track in the list and inserts new
    /// entity to the [`MusicPlaybacks`] resource as well
    fn keep_playlist_playing(
        _: On<Despawn, SamplePlayer>,
        mood: Res<State<Mood>>,
        settings: Res<Settings>,
        mut commands: Commands,
        mut sources: ResMut<AudioSources>,
        mut music_pbs: ResMut<MusicPlaybacks>,
    ) {
        let mood = mood.get();
        let mut rng = rand::rng();
        let handle = match mood {
            Mood::Exploration => sources.explore.pick(&mut rng),
            Mood::Combat => sources.combat.pick(&mut rng),
        };

        let id = commands
            .spawn((
                MusicPool,
                SamplePlayer::new(handle.clone())
                    .with_volume(settings.music())
                    .looping(),
                sample_effects![VolumeNode {
                    volume: Volume::SILENT,
                    ..default()
                }],
                FadeIn,
                *mood,
            ))
            .id();

        music_pbs.insert(*mood, id);
    }
}
