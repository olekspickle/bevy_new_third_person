//! The screen state for the main gameplay.

use crate::prelude::*;
use bevy::{prelude::*, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_gameplay_ui);
}

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
}
