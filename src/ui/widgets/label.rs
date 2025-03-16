use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

/// # Example
/// ```rust,no_run
///
/// fn setup_label(commands: &mut Commands, font: Res<MyFont>) {
///   commands
///     .ui_root()
///     .with_children(|children| {
///       let text = TextOpts::default()
///         .with_text("Play")
///         .with_font(TextFont {
///             font: font.0.clone(),
///             font_size: FONT_SIZE,
///             ..default()
///       });
///       children.label(&text, LayoutOpts::label());
///     });
/// }
///
/// ```
pub trait Labelable {
    fn label(
        &mut self,
        text_opts: impl Into<TextOpts>,
        layout: impl Into<LayoutOpts>,
    ) -> EntityCommands;
}

impl<T: Spawn> Labelable for T {
    fn label(
        &mut self,
        text_opts: impl Into<TextOpts>,
        layout: impl Into<LayoutOpts>,
    ) -> EntityCommands {
        let (text_opts, layout) = (text_opts.into(), layout.into());
        let s = text_opts.text.clone();
        let short = if s.len() > 10 { &s[..9] } else { &s };
        let entity = self.spawn((
            Name::new(format!("Label {short}")),
            Label,
            Text(s),
            layout.node.clone(),
            text_opts.font.clone(),
            TextColor(layout.color),
            BackgroundColor(layout.bg_color),
        ));
        entity
    }
}
