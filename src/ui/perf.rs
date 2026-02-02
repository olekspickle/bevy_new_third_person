use super::*;
use bevy_perf_ui::{
    PerfUiPlugin,
    entries::{PerfUiFramerateEntries, PerfUiWindowEntries},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_plugins((
        PerfUiPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        #[cfg(feature = "dev_native")]
        (
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        ),
        // https://github.com/IyesGames/iyes_perf_ui/issues/30
        // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
    ));

    app.add_systems(Startup, setup_perf_ui);
}

fn setup_perf_ui(mut commands: Commands) {
    commands.spawn((
        PerfUi,
        Node {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
        PerfUiRoot {
            position: PerfUiPosition::TopRight,
            ..default()
        },
        // Contains everything related to FPS and frame time
        PerfUiFramerateEntries::default(),
        // Contains everything related to the window and cursor
        PerfUiWindowEntries::default(),
        // Contains everything related to system diagnostics (CPU, RAM)
        // PerfUiSystemEntries::default(),
    ));
}
