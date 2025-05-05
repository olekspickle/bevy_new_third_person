use bevy::{prelude::*, ui::Val::*};
use iyes_perf_ui::{
    PerfUiPlugin,
    entries::{PerfUiFramerateEntries, PerfUiWindowEntries},
    prelude::*,
};

mod interaction;
mod opts;
mod palette;
mod widget;

pub use interaction::{InteractionPalette, OnPress};
pub use opts::*;
pub use palette::*;
pub use widget::*;

pub fn plugin(app: &mut App) {
    //app.load_resource_from_path::<Fira>("fonts/FiraCode-Regular.ttf");

    app.add_plugins((
        PerfUiPlugin,
        bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        // https://github.com/IyesGames/iyes_perf_ui/issues/30
        // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        interaction::plugin,
    ));

    app.add_systems(Startup, setup_perf_ui);
}

#[derive(Component)]
pub struct PerfUiMarker;

#[derive(Clone, Debug, Reflect, Asset, Resource)]
pub struct Fira(pub Handle<Font>);

fn setup_perf_ui(mut commands: Commands) {
    commands.spawn((
        PerfUiMarker,
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
