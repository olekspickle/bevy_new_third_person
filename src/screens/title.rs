use super::*;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup_menu)
        .add_systems(
            Update,
            btn_sounds
                .run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        );
}

fn setup_menu(
    //font: Res<Fira>,
    mut commands: Commands,
) {
    commands
        .spawn((
            ui_root("Title"),
            // Crutch until we can use #cfg in children![] macro
            // https://github.com/bevyengine/bevy/issues/18953
            #[cfg(target_family = "wasm")]
            children![
                btn("Play", to_gameplay),
                btn("Credits", to_credits),
                btn("Settings", to_settings),
            ],
            #[cfg(not(target_family = "wasm"))]
            children![
                btn("Play", to_gameplay),
                btn("Credits", to_credits),
                btn("Settings", to_settings),
                btn("Exit", exit_app)
            ],
        ))
        .insert(StateScoped(Screen::Title));
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}

fn btn_sounds(
    mut commands: Commands,
    settings: Res<Settings>,
    audio_sources: Res<AudioSources>,
    interaction_query: Query<&Interaction, Changed<Interaction>>,
) {
    for interaction in &interaction_query {
        let source = match interaction {
            Interaction::Hovered => audio_sources.btn_hover.clone(),
            Interaction::Pressed => audio_sources.btn_press.clone(),
            _ => continue,
        };
        let vol = settings.sound.general * settings.sound.sfx;
        commands.spawn(sfx(source, vol));
    }
}
