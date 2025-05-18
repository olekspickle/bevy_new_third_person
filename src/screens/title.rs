use super::*;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup_menu);
}

fn setup_menu(
    //font: Res<Fira>,
    mut commands: Commands,
) {
    commands.spawn((
        StateScoped(Screen::Title),
        ui_root("Title"),
        // Crutch until we can use #cfg in children![] macro
        // https://github.com/bevyengine/bevy/issues/18953
        #[cfg(target_family = "wasm")]
        children![
            btn("Play", to_gameplay_or_loading),
            btn("Credits", to_credits),
            btn("Settings", to_settings),
        ],
        #[cfg(not(target_family = "wasm"))]
        children![
            btn("Play", to_gameplay_or_loading),
            btn("Credits", to_credits),
            btn("Settings", to_settings),
            btn("Exit", exit_app)
        ],
    ));
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
