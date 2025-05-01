use crate::prelude::*;
use bevy::prelude::*;

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
    let opts = Opts::new("Play").with_bg_color(Color::WHITE);
    commands
        .spawn((
            ui_root("loading"),
            children![
                button(opts.clone(), enter_gameplay_screen),
                #[cfg(not(target_family = "wasm"))]
                button(opts.with_text("Exit"), exit_app)
            ],
        ))
        .insert(StateScoped(Screen::Title));
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
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
        commands.spawn(sound_effect(source));
    }
}
