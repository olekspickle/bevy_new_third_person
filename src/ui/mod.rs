use crate::prelude::*;
use bevy::{prelude::*, ui::Val::*};
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
        interaction::plugin,
        widgets::plugin,
        PerfUiPlugin,
    ));

    app.add_systems(OnEnter(Screen::Gameplay), setup);
}

fn setup(mut commands: Commands, camera: Query<Entity, With<Camera3d>>) {
    let camera = camera.single();
    commands.ui_root().spawn((
        TargetCamera(camera),
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
