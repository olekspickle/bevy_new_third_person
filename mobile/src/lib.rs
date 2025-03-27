use bevy::{prelude::*, window::WindowMode, winit::WinitSettings};
use bevy_new_third_person::game; // ToDo: Replace with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WinitSettings::mobile())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    // on iOS, gestures must be enabled.
                    #[cfg(target_os = "ios")]
                    recognize_rotation_gesture: true,
                    #[cfg(target_os = "ios")]
                    recognize_pinch_gesture: true,
                    ..default()
                }),
                ..default()
            }),
            game,
        ))
        .run();
}
