//! The screen state for the main gameplay.

use super::*;
use bevy::audio::Volume;
use bevy_third_person_camera::ThirdPersonCamera;
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<OnMenuToggle>()
        .add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(
            OnEnter(Screen::Gameplay),
            (
                start_or_resume_soundtrack.after(scene::setup),
                spawn_gameplay_ui,
            ),
        )
        .add_systems(
            Update,
            (toot, movement_sound, invoke_menu)
                .run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_observer(toggle_menu);
}

#[derive(Component)]
pub struct DevTools;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;
#[derive(Component)]
pub struct SunCycleLabel;
// TODO: The idea is to create a boombox with spatial audio
// <https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_3d.rs>
// #[derive(Component)]
// pub struct Boombox;

fn spawn_gameplay_ui(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DevTools,
        Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![
            // Demo keys
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
                    label("V - incr fov"),
                    label(format!("O - toggle sun cycle:{:?}", settings.sun_cycle)),
                    TextLayout::new_with_justify(JustifyText::Left)
                ]
            ),
            // Demo title
            (
                ui_root("game name"),
                children![label("{{ project-name }}"),]
            )
        ],
    ));
}

fn toot(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Toot) {
        let vol = settings.sound.general * settings.sound.sfx;
        commands.spawn(sfx(sources.btn_hover.clone(), vol));
    }

    Ok(())
}

#[derive(Event)]
pub struct OnMenuToggle;
#[derive(Component)]
pub struct Menu;

fn invoke_menu(mut commands: Commands, action: Query<&ActionState<Action>>) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Menu) {
        commands.trigger(OnMenuToggle);
    }

    Ok(())
}

fn trigger_toggle_menu(_: Trigger<OnPress>, mut commands: Commands) {
    commands.trigger(OnMenuToggle);
}

fn toggle_menu(
    _: Trigger<OnMenuToggle>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    menu: Query<Entity, With<Menu>>,
    mut cam: Query<&mut ThirdPersonCamera>,
) {
    let Ok(mut cam) = cam.single_mut() else {
        return;
    };
    cam.cursor_lock_active = !cam.cursor_lock_active;
    settings.menu_open = !settings.menu_open;

    if settings.menu_open {
        commands.spawn((
            Menu,
            ui_root("In game menu"),
            children![
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::End,
                    ..Default::default()
                },
                children![
                    Node { ..default() },
                    btn_small("x", trigger_toggle_menu),
                    btn("Settings", to_settings),
                    btn("Main Menu", to_title)
                ]
            ],
        ));
    } else if let Ok(menu) = menu.single() {
        commands.entity(menu).despawn();
    }
}

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
