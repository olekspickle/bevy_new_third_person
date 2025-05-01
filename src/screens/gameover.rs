use crate::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_screen);
}

fn spawn_screen(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            ui_root("game over screen"),
            children![
                label(format!("Score: {}", score.0)),
                button("PlayAgain", enter_gameplay_screen),
                #[cfg(not(target_family = "wasm"))]
                button("Exit", exit_app)
            ],
        ))
        .insert(StateScoped(Screen::GameOver));
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
