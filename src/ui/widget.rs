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
            row_gap: Percent(2.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

pub fn text(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    (
        BackgroundColor(opts.bg_color),
        Text(opts.text.to_string()),
        TextColor(opts.color),
        opts.font.clone(),
        opts.text_layout,
        // Don't bubble picking events from the text up to parent
        Pickable::IGNORE,
    )
}

pub fn label(opts: impl Into<Opts>) -> impl Bundle {
    let opts = opts.into();
    let s = opts.text.clone();
    let short = if s.len() > 10 { &s[..8] } else { &s };

    (
        Label,
        Name::new(format!("Label {short}")),
        BorderRadius::all(Px(opts.border_radius)),
        text(opts),
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(opts: impl Into<Opts>) -> impl Bundle {
    let mut opts = opts.into();
    opts.font.font_size = 40.0;
    (Label, Name::new("Header"), text(opts))
}

// A regular wide button with text and an action defined as an [`Observer`].
pub fn btn<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts = opts.into().node(Node {
        width: Px(30.0),
        height: Px(30.0),
        min_width: Px(MIN_WIDTH),
        padding: UiRect::all(Px(50.0)),
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
    let opts = opts.into().node(Node {
        width: Px(30.0),
        height: Px(30.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    });

    btn_base(opts, action)
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
/// Background color is set by [`InteractionPalette`]
pub fn btn_base<E, B, M, I>(opts: impl Into<Opts>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts = opts.into();
    let action = IntoObserverSystem::into_system(action);

    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    BorderRadius::all(Px(opts.border_radius)),
                    BorderColor(opts.border_color),
                    InteractionPalette {
                        none: BLUE,
                        hovered: DIM_BLUE,
                        pressed: LIGHT_BLUE,
                    },
                    children![Name::new("Button text"), text(opts.clone())],
                ))
                .insert(opts.node)
                .observe(action);
        })),
    )
}
