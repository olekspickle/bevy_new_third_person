use crate::prelude::*;
use bevy::{prelude::*, ui::Val::*};

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Menu), setup_menu);
}

fn setup_menu(
    //font: Res<Fira>,
    mut commands: Commands,
    camera: Query<Entity, With<SceneCamera>>,
) {
    let camera = camera.single();
    commands
        .ui_root()
        .insert(TargetCamera(camera))
        .insert(StateScoped(Screen::Menu))
        .with_children(|children| {
            let layout = LayoutOpts::button().with_node(Node {
                padding: UiRect::all(Px(10.0)),
                ..Default::default()
            });
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
