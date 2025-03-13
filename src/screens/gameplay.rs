//! The screen state for the main gameplay.

use crate::{
    prelude::*,
    ui::{Label, UiRoot},
};
use bevy::{prelude::*, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_gameplay_ui);
}

fn spawn_gameplay_ui(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label(&LabelOpts {
                text: "Hello Third Person".into(),
                node: Node {
                    top: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
