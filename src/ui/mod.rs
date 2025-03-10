use bevy::{prelude::*, ui::Val::*};
use iyes_perf_ui::{
    PerfUiPlugin,
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
};

mod button;
mod label;
pub use button::{ButtonOpts, Buttonable};
pub use label::{Label, LabelOpts};

pub const FONT_SIZE: f32 = 24.0;
pub const COLOR_NORM: Color = Color::srgb(0.9, 0.9, 0.9);
pub const BG_COLOR_NORM: Color = Color::srgb(0.1, 0.1, 0.4);
pub const BG_COLOR_HOVER: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BG_COLOR_PRESSED: Color = Color::srgb(0.35, 0.75, 0.35);
pub const NODE_BACKGROUND: Color = Color::srgb(0.286, 0.478, 0.773);

pub fn plugin(app: &mut App) {
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin);

    app.add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.spawn((
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

#[derive(Clone, Debug, Reflect, Asset, Resource)]
pub struct Fira(pub Handle<Font>);

/// Event triggered on a UI entity when the [`Interaction`] component on the same entity changes to
/// [`Interaction::Pressed`]. Observe this event to detect e.g. button presses.
#[derive(Event)]
pub struct OnPress;

/// (courtesy of RynKitty)
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
pub trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}

impl Spawn for EntityCommands<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.insert(bundle).reborrow()
    }
}

/// An extension trait for spawning UI containers. (courtesy of RynKitty)
pub trait UiRoot {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl UiRoot for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                row_gap: Px(10.0),
                ..default()
            },
        ))
    }
}
