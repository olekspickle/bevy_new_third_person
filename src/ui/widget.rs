//! Helper functions for creating common widgets.

use super::*;
use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    ui::Val::*,
};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Px(10.0),
            ..default()
        },
    )
}

pub fn text(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    (
        Text(opts.text.to_string()),
        TextColor(opts.color),
        BackgroundColor(opts.bg_color),
        opts.text_layout,
    )
}

pub fn label(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    let s = opts.text.clone();

    // dynamic width calculated from font size so that label node does not cut it out
    let short = if s.len() > 10 { &s[..8] } else { &s };
    let node = Node {
        width: Px(s.len() as f32 * opts.font.font_size),
        ..opts.node.clone()
    };

    (
        Name::new(format!("Label {short}")),
        opts.font.clone(),
        Label,
        node,
        text(opts),
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
pub fn button<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let action = IntoObserverSystem::into_system(action);
    let opts = opts.into();
    (
        Name::new(format!("Button {}", opts.text)),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    opts.node.clone(),
                    BorderRadius::all(Px(opts.border_radius)),
                    BorderColor(opts.border_color),
                    // Background color is set here
                    InteractionPalette {
                        none: BTN_BG,
                        hovered: BTN_HOVERED_BG,
                        pressed: BTN_PRESSED_BG,
                    },
                    text(opts),
                ))
                .observe(action);
        })),
    )
}
