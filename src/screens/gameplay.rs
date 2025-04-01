//! The screen state for the main gameplay.

use crate::prelude::*;
use bevy::{prelude::*, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_gameplay_ui);
}

#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;

fn spawn_gameplay_ui(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Gameplay))
        .with_children(|children| {
            let opts = LayoutOpts::label().with_node(Node {
                top: Px(10.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            });
            children.label("Hello Third Person", opts);
        });

    // Demo keys
    commands
        .container(Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        })
        .with_children(|children| {
            ChildBuild::spawn(
                children,
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
            )
            .with_children(|children| {
                let layout = LayoutOpts::label().with_node(Node {
                    justify_items: JustifyItems::Start,
                    ..Default::default()
                });
                children
                    .label("P - pause", layout.clone())
                    .spawn(PauseLabel);
                children.label("M - mute", layout.clone()).spawn(MuteLabel);
                children.label("F - diagnostics", layout);
            });
        });
}
