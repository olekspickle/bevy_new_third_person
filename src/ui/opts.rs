use super::*;
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
        let text = s.into();
        // a bit of a hack IMO - it's weird that text node is not the width of the text by default
        // let min_width = Px(text.len() as f32 * FONT_SIZE / 1.2);
        Self {
            text,
            text_layout: TextLayout::new_with_justify(JustifyText::Center),
            font: TextFont::from_font_size(FONT_SIZE),
            node: Node {
                // min_width,
                // width: Percent(30.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Px(2.0)),
                padding: UiRect::horizontal(Px(10.0)),
                ..Default::default()
            },
            color: WHITEISH,
            bg_color: TRANSPARENT,
            border_color: WHITEISH,
            border_radius: BORDER_RADIUS,
        }
    }

    pub(crate) fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.text = text.into();
        self
    }
    pub(crate) fn text_layout(mut self, layout: TextLayout) -> Self {
        self.text_layout = layout;
        self
    }
    pub(crate) fn font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub(crate) fn size(mut self, s: f32) -> Self {
        self.font.font_size = s;
        self
    }
    pub(crate) fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub(crate) fn bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }
    pub(crate) fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    pub(crate) fn border_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }
    pub(crate) fn node(mut self, new: Node) -> Self {
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
    pub(crate) fn width(mut self, w: f32) -> Self {
        self.node.width = Px(w);
        self
    }
    pub(crate) fn height(mut self, h: f32) -> Self {
        self.node.height = Px(h);
        self
    }
    pub(crate) fn margin(mut self, m: UiRect) -> Self {
        self.node.margin = m;
        self
    }
    pub(crate) fn padding(mut self, p: UiRect) -> Self {
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
