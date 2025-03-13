use super::*;
use bevy::ecs::system::EntityCommands;

#[derive(Debug, Clone)]
pub struct LabelOpts {
    pub text: String,
    pub color: Color,
    pub bg_color: Color,
    pub font: TextFont,
    pub node: Node,
}

impl LabelOpts {
    pub fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub fn with_label(mut self, s: impl Into<String>) -> Self {
        self.text = s.into();
        self
    }
    pub fn with_node(mut self, n: Node) -> Self {
        self.node = n;
        self
    }
}

// For something like "my-label".into()
impl<T: Into<String>> From<T> for LabelOpts {
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            ..default()
        }
    }
}

impl From<&ButtonOpts> for LabelOpts {
    fn from(value: &ButtonOpts) -> Self {
        Self {
            text: value.label_opts.text.clone(),
            node: Node {
                height: Px(value.font.font_size),
                ..value.node.clone()
            },
            color: value.color,
            bg_color: value.bg_color,
            font: value.font.clone(),
        }
    }
}

impl Default for LabelOpts {
    fn default() -> Self {
        Self {
            node: Node {
                width: Px(150.0),
                height: Px(FONT_SIZE),
                border: UiRect::ZERO,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: COLOR_NORM,
            bg_color: BG_COLOR_NORM,
            text: "label".into(),
            font: TextFont::from_font_size(FONT_SIZE),
        }
    }
}

pub trait Label {
    fn label(&mut self, opts: &LabelOpts) -> EntityCommands;
}

impl<T: Spawn> Label for T {
    fn label(&mut self, opts: &LabelOpts) -> EntityCommands {
        let s = opts.text.clone();
        let short = if s.len() > 10 { &s[..9] } else { &s };
        let entity = self.spawn((
            Name::new(format!("Label {short}")),
            Text(s),
            opts.node.clone(),
            opts.font.clone(),
            TextColor(opts.color),
            BackgroundColor(opts.bg_color),
        ));
        entity
    }
}
