use super::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_screen);
}

fn spawn_screen(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            ui_root("game over screen"),
            #[cfg(target_family = "wasm")]
            children![
                label(format!("Score: {}", score.0)),
                btn("PlayAgain", enter_gameplay_screen),
            ],
            #[cfg(not(target_family = "wasm"))]
            children![
                label(format!("Score: {}", score.0)),
                btn("PlayAgain", enter_gameplay_screen),
                btn("Exit", exit_app)
            ],
        ))
        .insert(StateScoped(Screen::GameOver));
}

fn enter_gameplay_screen(
    _trigger: Trigger<Pointer<Click>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
