use super::*;

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State [`Screen::Title`] and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), (setup_menu, start_main_menu_music));
}

fn setup_menu(
    main_menu_ctx: Single<Entity, With<MainMenuCtx>>,
    mut commands: Commands,
    mut modals: ResMut<Modals>,
    mut state: ResMut<GameState>,
) {
    commands.spawn((
        DespawnOnExit(Screen::Title),
        widget::ui_root("Title UI"),
        BackgroundColor(colors::TRANSLUCENT),
        children![(
            Name::new("Title menu"),
            Node {
                width: Vw(40.0),
                height: Vh(40.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Vh(1.0),
                bottom: Vw(1.0),
                left: Vw(1.0),
                ..default()
            },
            // Crutch until we can use #cfg in children![] macro
            // https://github.com/bevyengine/bevy/issues/18953
            #[cfg(feature = "web")]
            children![
                (widget::btn_big("Play", click_go_to), Screen::Gameplay),
                (widget::btn_big("Credits", click_go_to), Screen::Credits),
                (widget::btn_big("Settings", click_go_to), Screen::Settings),
            ],
            #[cfg(not(feature = "web"))]
            children![
                (widget::btn_big("Play", click_go_to), Screen::Gameplay),
                (widget::btn_big("Credits", click_go_to), Screen::Credits),
                (widget::btn_big("Settings", click_go_to), Screen::Settings),
                widget::btn_big("Exit", exit_app)
            ],
        )],
    ));

    // re-enter sanitation
    state.reset();
    modals.clear();
    commands.entity(*main_menu_ctx).insert(ModalInput);

    state.reset();
}

fn start_main_menu_music(
    settings: Res<Settings>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
    mut music: Query<&mut PlaybackSettings, With<MusicPool>>,
) {
    for mut s in music.iter_mut() {
        s.pause();
    }

    let handle = sources.menu.pick(&mut rand::rng());
    commands.spawn((
        DespawnOnExit(Screen::Title),
        Name::new("Title Music"),
        MusicPool,
        SamplePlayer::new(handle.clone())
            .with_volume(settings.music())
            .looping(),
    ));
}

#[cfg(not(feature = "web"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
