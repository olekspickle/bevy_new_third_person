use bevy::prelude::*;
use iyes_perf_ui::{
    PerfUiPlugin,
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
    prelude::PerfUiAllEntries,
};

mod interaction;
mod opts;
mod palette;
mod widget;

pub use interaction::{InteractionPalette, OnPress};
pub use opts::*;
pub use palette::*;
pub use widget::*;

pub const FONT_SIZE: f32 = 24.0;
pub const BORDER_RADIUS: f32 = 15.0;
pub const MIN_WIDTH: f32 = 200.0;

pub fn plugin(app: &mut App) {
    //app.load_resource_from_path::<Fira>("fonts/FiraCode-Regular.ttf");

    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        bevy::diagnostic::EntityCountDiagnosticsPlugin,
        bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        bevy::render::diagnostic::RenderDiagnosticsPlugin,
        PerfUiPlugin,
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
        ui_root("UI Perf"),
        PerfUiMarker,
        children![
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::End,
                ..Default::default()
            },
            children![
                // Contains everything related to FPS and frame time
                PerfUiFramerateEntries::default(),
                // Contains everything related to the window and cursor
                PerfUiWindowEntries::default(),
                // Contains everything related to system diagnostics (CPU, RAM)
                PerfUiSystemEntries::default(),
                // Contains everything related to fixed timestep
                PerfUiFixedTimeEntries::default(),
            ]
        ],
    ));
}
