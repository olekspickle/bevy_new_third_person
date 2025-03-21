//! Helper traits for creating common widgets.

use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

pub mod button;
pub mod label;

pub use button::Buttonable;
pub use label::Labelable;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Debug, Clone)]
pub struct LayoutOpts {
    pub border_radius: f32,
    pub border_color: Color,
    pub bg_color: Color,
    pub color: Color,
    pub node: Node,
}

impl LayoutOpts {
    pub fn button() -> Self {
        Self {
            node: Node {
                width: Px(150.0),
                height: Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Px(2.0)),
                ..Default::default()
            },
            color: BTN,
            bg_color: BTN_BG,
            border_color: BTN_BG,
            border_radius: BORDER_RADIUS,
        }
    }

    pub fn label() -> Self {
        Self {
            node: Node {
                width: Px(150.0),
                height: Px(FONT_SIZE),
                border: UiRect::ZERO,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: LABEL,
            bg_color: LABEL_BG,
            border_color: LABEL_BG,
            border_radius: BORDER_RADIUS,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    pub fn with_node(mut self, n: Node) -> Self {
        self.node = n;
        self
    }
    pub fn with_border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct TextOpts {
    text: String,
    font: TextFont,
}

impl TextOpts {
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
    pub fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
}

// For something like "my-label".into()
impl<T: Into<String>> From<T> for TextOpts {
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            font: TextFont::from_font_size(FONT_SIZE),
        }
    }
}

pub trait GenericContainer {
    /// Spawns a container node with specified direction, align_items, and justify_content.
    fn container(
        &mut self,
        direction: FlexDirection,
        align_items: AlignItems,
        justify_content: JustifyContent,
    ) -> EntityCommands;
}

impl<T: Spawn> GenericContainer for T {
    fn container(
        &mut self,
        direction: FlexDirection,
        align_items: AlignItems,
        justify_content: JustifyContent,
    ) -> EntityCommands {
        self.spawn((
            Name::new("Container"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                justify_content,
                align_items,
                flex_direction: direction,
                ..default()
            },
        ))
    }
}
