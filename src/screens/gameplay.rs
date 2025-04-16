//! The screen state for the main gameplay.
//!
use crate::prelude::*;
use bevy::{audio::Volume, prelude::*, ui::Val::*};
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
        .ui_root()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|children| {
            let opts = LayoutOpts::label().with_node(Node {
                top: Px(10.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            });
            children.label("Hello Third Person", opts);
        });

    // Demo keys
    commands
        .container(Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        })
        .with_children(|children| {
            ChildBuild::spawn(
                children,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
            )
            .with_children(|children| {
                let layout = LayoutOpts::label().with_node(Node {
                    justify_items: JustifyItems::Start,
                    ..Default::default()
                });

                children
                    .label("P - pause", layout.clone())
                    .spawn((PauseLabel, TextLayout::new_with_justify(JustifyText::Left)));
                children
                    .label("M - mute", layout.clone())
                    .spawn((MuteLabel, TextLayout::new_with_justify(JustifyText::Left)));
                children
                    .label("F - diagnostics", layout)
                    .spawn(TextLayout::new_with_justify(JustifyText::Left));
            });
        });
}

fn invoke_settings(
    action: Query<&ActionState<Action>>,
    // mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    // mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) {
    let state = action.single();
    if state.just_pressed(&Action::Settings) {}
}

fn start_or_resume_bg_music(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    music: Query<&mut AudioSink, With<Music>>,
) {
    if let Ok(music) = music.get_single() {
        if music.is_paused() {
            // TODO: use seedling under feature
            music.toggle();
        } else {
            let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
            commands.spawn((
                Music,
                AudioPlayer::new(handle.clone()),
                PlaybackSettings {
                    volume: Volume::new(settings.sound.general * settings.sound.music),
                    ..Default::default()
                },
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
    action: Query<&ActionState<Action>>,
    position: Query<&Transform, With<Player>>,
) {
    let (player_pos, state) = (position.single(), action.single());
    let Ok(mut step_timer) = step_timer.get_single_mut() else {
        return;
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
                    volume: Volume::new(settings.sound.general * settings.sound.sfx),
                    ..Default::default()
                },
            ));
        }
    }
}
