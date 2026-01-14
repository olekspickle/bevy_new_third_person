//! An abstraction for changing music of the game depending on some triggers
use super::*;
use crate::player::Player;
use avian3d::prelude::Collisions;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack)
        .add_systems(
            Update,
            trigger_mood_change
                .run_if(in_state(Screen::Gameplay))
                .run_if(on_timer(Duration::from_millis(200))),
        )
        .add_observer(change_mood);
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
    mut music_pb: ResMut<MusicPlaybacks>,
) {
    for (_, e) in music_pb.iter_mut() {
        let Ok(mut s) = music.get_mut(*e) else {
            continue;
        };
        s.pause();
    }
}

fn trigger_mood_change(
    collisions: Collisions,
    state: ResMut<GameState>,
    zones: Query<(Entity, &Mood)>,
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
) {
    let Ok(player) = player.single_mut() else {
        return;
    };
    for (e, zone) in zones.iter() {
        if collisions.contains(player, e) {
            info!("colliding with zone: {:?}", zone);
            match zone {
                Mood::Combat => {
                    if state.current_mood != Mood::Combat {
                        debug!("Trigger changing mood from:{:?}", state.current_mood);
                        commands.trigger(ChangeMood {
                            mood: Mood::Combat,
                            entity: player,
                        });
                    }
                }
                Mood::Exploration => {
                    if state.current_mood != Mood::Exploration {
                        debug!("Trigger changing mood from:{:?}", state.current_mood);
                        commands.trigger(ChangeMood {
                            mood: Mood::Exploration,
                            entity: player,
                        })
                    }
                }
            }
        }
    }
}

/// Every time the current mood in GameState resource changes,
/// this system is run to trigger the song change
fn change_mood(
    on: On<ChangeMood>,
    settings: Res<Settings>,
    music_pb: ResMut<MusicPlaybacks>,
    mut commands: Commands,
    mut state: ResMut<GameState>,
    mut sources: ResMut<AudioSources>,
) {
    let mut rng = rand::rng();
    for (z, track) in music_pb.iter() {
        if z != &on.mood {
            commands.entity(*track).insert(FadeOut);
        }
    }
    state.current_mood = on.mood;

    if let Some(track) = music_pb.get(&on.mood) {
        debug!("found existing track, fading IN: {track}");
        commands.entity(*track).insert(FadeIn);
        return;
    }

    debug!("did not find existing track, spawning new for mood");
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
