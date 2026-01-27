use super::*;
use crate::ui::Interactable;
use bevy::ecs::system::IntoObserverSystem;
use std::borrow::Cow;

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
            row_gap: Vh(5.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

pub fn icon(opts: impl Into<Props>) -> impl Bundle {
    let opts = opts.into();
    (
        Label,
        Name::new("Icon"),
        opts.node.clone(),
        opts.border_radius,
        opts.into_image_bundle(),
    )
}
pub fn label(opts: impl Into<Props>) -> impl Bundle {
    let opts = opts.into();
    (
        Label,
        Name::new("Label"),
        opts.node.clone(),
        opts.border_radius,
        opts.into_text_bundle(),
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(opts: impl Into<Props>) -> impl Bundle {
    let opts = opts.into();
    (Label, Name::new("Header"), opts.into_text_bundle())
}

// A regular wide button with text and an action defined as an [`Observer`].
pub fn btn_big<E, B, M, I>(opts: impl Into<Props>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts: Props = opts.into();
    let new_node = Node {
        min_width: Vw(30.0),
        padding: UiRect::axes(Vw(8.0), Vh(2.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..opts.node.clone()
    };
    let opts = opts.node(new_node);

    btn(opts, action)
}

// A small square button with text and an action defined as an [`Observer`].
pub fn btn_small<E, B, M, I>(opts: impl Into<Props>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let opts: Props = opts.into();
    let new_node = Node {
        margin: UiRect::ZERO,
        padding: UiRect::horizontal(Vw(1.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..opts.node.clone()
    };
    let mut opts = opts.node(new_node);
    opts.border_radius = BorderRadius::all(Px(7.0));

    btn(opts, action)
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
/// Background color is set by [`UiPalette`]
pub fn btn<E, B, M, I>(opts: impl Into<Props>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let mut opts: Props = opts.into();
    let action = IntoObserverSystem::into_system(action);

    (
        Button,
        Interactable,
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            let content = match &opts.content {
                WidgetContent::Image(_) => parent
                    .spawn((opts.clone().into_image_bundle(), Pickable::IGNORE))
                    .id(),
                WidgetContent::Text(_) => parent
                    .spawn((opts.clone().into_text_bundle(), Pickable::IGNORE))
                    .id(),
            };
            opts.node.width = Percent(100.0);
            opts.node.height = Percent(100.0);

            parent
                .spawn((
                    opts.bg_color,
                    opts.border_radius,
                    opts.border_color,
                    opts.palette_set.clone(),
                    Name::new("Button Content"),
                ))
                .insert(opts.node)
                .add_children(&[content])
                .observe(action);
        })),
    )
}

// courtesy of @jannhohenheim
pub(crate) fn plus_minus_bar<E, B, M, I1, I2>(
    label_marker: impl Component,
    lower: I1,
    raise: I2,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I1: IntoObserverSystem<E, B, M>,
    I2: IntoObserverSystem<E, B, M>,
{
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            btn_small("-", lower),
            (label(""), label_marker),
            btn_small("+", raise),
        ],
    )
}
