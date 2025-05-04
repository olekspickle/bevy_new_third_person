use super::*;
use bevy::ui::Val::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Opts {
    // text
    pub text_layout: TextLayout,
    pub text: Cow<'static, str>,
    pub font: TextFont,
    // layout
    pub border_radius: f32,
    pub border_color: Color,
    pub bg_color: Color,
    pub color: Color,
    pub node: Node,
}

impl Opts {
    pub(crate) fn new(s: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: s.into(),
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            font: TextFont::from_font_size(FONT_SIZE),
            node: Node {
                width: Percent(30.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Px(2.0)),
                padding: UiRect::horizontal(Px(10.0)),
                min_width: Px(MIN_WIDTH),
                ..Default::default()
            },
            color: WHITEISH,
            bg_color: TRANSPARENT,
            border_color: WHITEISH,
            border_radius: BORDER_RADIUS,
        }
    }

    pub(crate) fn btn(s: impl Into<Cow<'static, str>>) -> Self {
        let mut new = Self::new(s);
        new.node.padding = UiRect::all(Px(10.0));
        new
    }

    pub(crate) fn with_text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.text = text.into();
        self
    }
    pub(crate) fn with_text_layout(mut self, layout: TextLayout) -> Self {
        self.text_layout = layout;
        self
    }
    pub(crate) fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub(crate) fn with_size(mut self, s: f32) -> Self {
        self.font.font_size = s;
        self
    }
    pub(crate) fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub(crate) fn with_bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }
    pub(crate) fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    pub(crate) fn with_border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }
    pub(crate) fn with_node(mut self, new: Node) -> Self {
        let Node {
            width,
            height,
            align_items,
            align_content,
            justify_items,
            justify_content,
            border,
            padding,
            ..
        } = self.node;
        self.node = Node {
            height,
            width,
            align_items,
            align_content,
            justify_items,
            justify_content,
            border,
            padding,
            min_width: Px(MIN_WIDTH),
            ..new
        };
        self
    }
    pub(crate) fn with_margin(mut self, m: UiRect) -> Self {
        self.node.margin = m;
        self
    }
    pub(crate) fn with_padding(mut self, p: UiRect) -> Self {
        self.node.padding = p;
        self
    }
}

// For something like "my-label".into()
impl<T: Into<Cow<'static, str>>> From<T> for Opts {
    fn from(value: T) -> Self {
        Opts::new(value)
    }
}
