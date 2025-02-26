use bevy::prelude::*;

use crate::{Score, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    //app.add_systems(OnEnter(Screen::GameOver), spawn_screen);
}

//fn spawn_screen(mut commands: Commands, score: Res<Score>) {
//    println!("GAME OVER");
//commands
//    .ui_root()
//    .insert(StateScoped(Screen::GameOver))
//    .with_children(|children| {
//        children.label(format!("Score: {}", score.0));
//        children.button("PlayAgain").observe(enter_gameplay_screen);
//
//        #[cfg(not(target_family = "wasm"))]
//        children.button("Exit").observe(exit_app);
//    });
//}

//fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
//    next_screen.set(Screen::Playing);
//}
//
//#[cfg(not(target_family = "wasm"))]
//fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
//    app_exit.send(AppExit::Success);
//}
