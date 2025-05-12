//! Helper functions for creating common widgets.

use super::*;
use bevy::ecs::{spawn::SpawnWith, system::IntoObserverSystem};
use std::borrow::Cow;

pub const BORDER_RADIUS: f32 = 15.0;
pub const FONT_SIZE: f32 = 24.0;
pub const MIN_WIDTH: f32 = 200.0;

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
        // BackgroundColor(DEBUG_BLUE),
        BackgroundColor(opts.bg_color),
        opts.text_layout,
    )
}

pub fn label(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    let s = opts.text.clone();
    let short = if s.len() > 10 { &s[..8] } else { &s };

    (
        Name::new(format!("Label {short}")),
        BorderRadius::all(Px(opts.border_radius)),
        opts.font.clone(),
        opts.node.clone(),
        Label,
        text(opts),
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    (
        Name::new("Header"),
        Text(opts.text.into()),
        TextFont::from_font_size(40.0),
        TextColor(opts.color),
    )
}

// A regular wide button with text and an action defined as an [`Observer`].
pub fn btn<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts = opts.into().with_node(Node {
        width: Px(30.0),
        height: Px(30.0),
        min_width: Px(MIN_WIDTH),
        padding: UiRect::all(Px(10.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    });

    btn_base(opts, action)
}

// A small square button with text and an action defined as an [`Observer`].
pub fn btn_small<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts = opts.into().with_node(Node {
        width: Px(30.0),
        height: Px(30.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    });

    btn_base(opts, action)
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
pub fn btn_base<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts = opts.into();
    let action = IntoObserverSystem::into_system(action);

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
                        none: BLUE,
                        hovered: DIM_BLUE,
                        pressed: LIGHT_BLUE,
                    },
                    text(opts),
                ))
                .observe(action);
        })),
    )
}
