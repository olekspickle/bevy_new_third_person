//! A credits screen that can be accessed from the title screen.

use super::*;
use bevy::{ecs::spawn::SpawnIter, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Credits),
        (start_credits_music, spawn_credits_screen),
    );
}

fn spawn_credits_screen(mut commands: Commands, cfg: Res<Config>) {
    commands.spawn((
        ui_root("Credits Screen"),
        StateScoped(Screen::Credits),
        children![
            header("Created by"),
            flatten(&cfg.credits.devs),
            header("Assets"),
            flatten(&cfg.credits.assets),
            btn("Back", to_title),
        ],
    ));
}

fn flatten(devs: &[(String, String)]) -> impl Bundle {
    let devs: Vec<[String; 2]> = devs.iter().map(|(n, k)| [n.clone(), k.clone()]).collect();
    grid(devs)
}

fn grid(content: Vec<[String; 2]>) -> impl Bundle {
    let content = content.into_iter().flatten().enumerate().map(|(i, text)| {
        (
            label(text),
            Node {
                justify_self: if i % 2 == 0 {
                    JustifySelf::End
                } else {
                    JustifySelf::Start
                },
                ..default()
            },
        )
    });

    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content)),
    )
}

#[derive(Component, Default)]
struct CreditsMusic;

fn start_credits_music(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    mut bg_music: Query<&mut AudioSink, With<Music>>,
) {
    for s in bg_music.iter_mut() {
        s.pause();
    }
    let vol = settings.sound.general * settings.sound.music;
    let handle = sources.bg_music.clone();
    commands.spawn((
        StateScoped(Screen::Credits),
        Name::new("Credits Music"),
        music(handle, vol),
    ));
}
