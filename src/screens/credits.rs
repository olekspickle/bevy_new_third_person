//! A credits screen that can be accessed from the main menu
use super::*;
use bevy::ecs::spawn::SpawnIter;

const SCROLL_SPEED: f32 = 5.0; // 10px/s

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Credits),
        (start_credits_music, spawn_credits_screen),
    )
    .add_systems(Update, roll_the_credits.run_if(in_state(Screen::Credits)));
}

markers!(CreditsRoot);

fn spawn_credits_screen(mut commands: Commands, credits: Res<CreditsPreset>) {
    commands.spawn((
        DespawnOnExit(Screen::Credits),
        widget::ui_root("credits screen"),
        BackgroundColor(colors::TRANSLUCENT),
        children![(
            CreditsRoot,
            Node {
                width: Percent(100.0),
                position_type: PositionType::Absolute,
                bottom: Percent(-100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::scroll_y(),
                row_gap: Vh(5.0),
                ..default()
            },
            children![
                widget::header("Created by"),
                flatten(&credits.devs),
                widget::header("Assets"),
                flatten(&credits.assets),
                (widget::btn_big("Back", click_go_to), Screen::Title),
            ]
        )],
    ));
}

fn flatten(devs: &[(String, String)]) -> impl Bundle {
    let devs: Vec<[String; 2]> = devs.iter().map(|(n, k)| [n.clone(), k.clone()]).collect();
    grid(devs)
}

fn grid(content: Vec<[String; 2]>) -> impl Bundle {
    let content = content.into_iter().flatten().enumerate().map(|(i, text)| {
        (
            Text(text),
            Node {
                justify_self: if i.is_multiple_of(2) {
                    JustifySelf::End
                } else {
                    JustifySelf::Start
                },
                ..default()
            },
        )
    });

    (
        Name::new("Credits Grid"),
        Node {
            display: Display::Grid,
            row_gap: Vh(1.0),
            column_gap: Vw(5.0),
            grid_template_columns: RepeatedGridTrack::vw(2, 35.0),
            ..default()
        },
        Children::spawn(SpawnIter(content)),
    )
}

fn start_credits_music(
    settings: Res<Settings>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
    mut music: Query<&mut PlaybackSettings, With<MusicPool>>,
) {
    for mut s in music.iter_mut() {
        s.pause();
    }

    let handle = sources.explore.pick(&mut rand::rng());
    commands.spawn((
        DespawnOnExit(Screen::Credits),
        Name::new("Credits Music"),
        MusicPool,
        SamplePlayer::new(handle.clone())
            .with_volume(settings.music())
            .looping(),
    ));
}

fn roll_the_credits(time: Res<Time>, mut node: Single<&mut Node, With<CreditsRoot>>) {
    if let Percent(bottom) = node.bottom
        && bottom < 0.0
    {
        node.bottom = Percent(bottom + SCROLL_SPEED * time.delta_secs());
    }
}
