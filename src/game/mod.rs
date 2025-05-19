use crate::prelude::*;
use bevy::{audio::Volume, prelude::*};
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

pub mod camera;
pub mod player;
pub mod scene;
pub mod settings;
pub mod skybox;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        scene::plugin,
        player::plugin,
        skybox::plugin,
    ))
    .add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
    .add_systems(
        OnEnter(Screen::Gameplay),
        start_or_resume_soundtrack.after(scene::setup),
    );
}

#[derive(Default, Resource)]
pub struct Score(pub i32);

// TODO: implement different music states
// good structure in this example: <https://github.com/bevyengine/bevy/blob/main/examples/audio/soundtrack.rs#L29>
fn start_or_resume_soundtrack(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // boombox: Query<Entity, With<Boombox>>,
    mut music_query: Query<&mut AudioSink, With<Music>>,
) -> Result {
    if let Ok(instance) = music_query.single_mut() {
        if instance.is_paused() {
            // TODO: use seedling under feature
            instance.play();
        }
    } else {
        let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
        let vol = settings.sound.general * settings.sound.music;
        // // Play music from boombox entity
        // commands
        //     .entity(boombox.single()?)
        //     .insert(music(handle.clone(), vol));
        // Or just play music
        commands.spawn(music(handle.clone(), vol));
    }

    Ok(())
}

fn stop_soundtrack(
    // boombox: Query<Entity, With<Boombox>>,
    mut bg_music: Query<&mut AudioSink, With<Music>>,
) {
    for s in bg_music.iter_mut() {
        s.pause();
    }
}

fn movement_sound(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<Settings>,
    mut step_timer: Query<&mut StepTimer>,
    sources: ResMut<AudioSources>,
    state: Query<&ActionState<Action>>,
    player_pos: Query<&Transform, With<Player>>,
) -> Result {
    let Ok(player_pos) = player_pos.single() else {
        return Ok(());
    };
    let Ok(state) = state.single() else {
        return Ok(());
    };
    let Ok(mut step_timer) = step_timer.single_mut() else {
        return Ok(());
    };

    if step_timer.0.tick(time.delta()).just_finished() {
        // TODO: only run animation after tick
        if (state.pressed(&Action::Forward)
            | state.pressed(&Action::Backward)
            | state.pressed(&Action::Left)
            | state.pressed(&Action::Right))
            && player_pos.translation.y == 0.0
        {
            let mut rng = thread_rng();
            let i = rng.gen_range(0..sources.steps.len());
            let handle = sources.steps[i].clone();
            commands.spawn((
                SoundEffect,
                AudioPlayer::new(handle),
                PlaybackSettings {
                    volume: Volume::Linear(settings.sound.general * settings.sound.sfx),
                    ..Default::default()
                },
            ));
        }
    }

    Ok(())
}
