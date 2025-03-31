use bevy::prelude::*;
use iyes_perf_ui::{
    PerfUiPlugin,
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
};

mod interaction;
mod palette;
mod widgets;

pub use interaction::{InteractionPalette, OnPress};
pub use palette::*;
pub use widgets::*;

pub const FONT_SIZE: f32 = 24.0;
pub const BORDER_RADIUS: f32 = 15.0;

pub fn plugin(app: &mut App) {
    //app.load_resource_from_path::<Fira>("fonts/FiraCode-Regular.ttf");

    app.add_plugins((
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
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
    commands
        .container(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,
            ..Default::default()
        })
        .spawn((
            PerfUiMarker,
            // Contains everything related to FPS and frame time
            PerfUiFramerateEntries::default(),
            // Contains everything related to the window and cursor
            PerfUiWindowEntries::default(),
            // Contains everything related to system diagnostics (CPU, RAM)
            PerfUiSystemEntries::default(),
            // Contains everything related to fixed timestep
            PerfUiFixedTimeEntries::default(),
        ));
}
