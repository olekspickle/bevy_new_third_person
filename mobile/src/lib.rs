use bevy::{prelude::*, window::WindowMode, WinitSettings};
use bevy_3rd_person_view::GamePlugin; // ToDo: Replace with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(WinitSettings::mobile())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
