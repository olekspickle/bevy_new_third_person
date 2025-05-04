//! The screen state for the main gameplay.

use crate::prelude::*;
use bevy::{audio::Volume, prelude::*};
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (start_or_resume_bg_music, spawn_gameplay_ui),
    )
    .add_systems(
        Update,
        movement_sound
            .run_if(resource_exists::<AudioSources>)
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;

fn spawn_gameplay_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            // Demo keys
            children![
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        ..Default::default()
                    },
                    children![
                        (label("P - pause"), PauseLabel),
                        (label("M - mute"), MuteLabel),
                        label("F - diagnostics"),
                        TextLayout::new_with_justify(JustifyText::Left)
                    ]
                ),
                // Demo title
                label("Hello Third Person"),
            ],
        ))
        .insert(StateScoped(Screen::Gameplay));
}

fn invoke_settings(
    action: Query<&ActionState<Action>>,
    // mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    // mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Settings) {}

    Ok(())
}

fn start_or_resume_bg_music(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    mut music_query: Query<&mut AudioSink, With<Music>>,
) {
    if let Ok(mut instance) = music_query.single_mut() {
        if instance.is_paused() {
            // TODO: use seedling under feature
            instance.toggle_mute();
        } else {
            let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
            commands.spawn(music(
                handle.clone(),
                settings.sound.general * settings.sound.music,
            ));
        }
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
