use super::*;
use bevy::ecs::system::EntityCommands;

#[derive(Debug, Clone)]
pub struct LabelOpts {
    text: String,
    color: Color,
    bg_color: Color,
    width: f32,
    height: f32,
    border: f32,
    font: TextFont,
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
            text: value.label.clone(),
            color: value.color,
            bg_color: value.bg_color,
            width: value.width,
            font: value.font.clone(),
            ..default()
        }
    }
}

impl Default for LabelOpts {
    fn default() -> Self {
        Self {
            width: 150.0,
            height: FONT_SIZE,
            color: COLOR_NORM,
            bg_color: BG_COLOR_NORM,
            text: "label".into(),
            font: TextFont::from_font_size(FONT_SIZE),
            ..default()
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
            opts.font.clone(),
            TextColor(opts.color),
            BackgroundColor(opts.bg_color),
            Node {
                width: Px(opts.width),
                height: Px(opts.height),
                ..default()
            },
        ));
        entity
    }
}
