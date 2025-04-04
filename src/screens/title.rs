use crate::prelude::*;
use bevy::prelude::*;
use bevy_seedling::prelude::*;

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
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            let layout = LayoutOpts::button();
            //let text = TextOpts::from("Play")
            //.with_font(TextFont {
            //    font: font.0.clone(),
            //    font_size: FONT_SIZE,
            //    ..default()
            //});
            children
                .button("Play", layout.clone())
                .observe(enter_gameplay_screen);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit", layout).observe(exit_app);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
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
        commands.spawn((
            SoundEffect,
            SamplePlayer::new(source),
            PlaybackSettings {
                volume: Volume::Linear(settings.sound.general),
                ..Default::default()
            },
        ));
    }
}
