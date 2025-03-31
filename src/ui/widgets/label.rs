use super::*;

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
///       // or if you don't care about font
///       children.label("Play", LayoutOpts::label());
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

        let short = if s.len() > 10 { &s[..8] } else { &s };
        let node = Node {
            // dynamic width calculated from font size
            width: Px(s.len() as f32 * text_opts.font.font_size),
            ..layout.node.clone()
        };
        let entity = self.spawn((
            Name::new(format!("Label {short}")),
            text_opts.font.clone(),
            node,
            Label,
            Text(s),
            TextColor(layout.color),
            BackgroundColor(layout.bg_color),
        ));
        entity
    }
}
