use super::*;

/// # Example
/// ```rust,no_run
///
/// fn setup_button(commands: &mut Commands, font: Res<MyFont>) {
///   commands
///     .ui_root()
///     .with_children(|children| {
///       let text = TextOpts::button()
///         .with_label("Play")
///         .with_font(TextFont {
///             font: font.0.clone(),
///             font_size: FONT_SIZE,
///             ..default()
///       });
///       children.button(text, LayoutOpts::button()).observe(go_to_play_screen);
///     });
/// }
///
///
/// fn go_to_play_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
///     next_screen.set(Screen::Playing);
/// }
///
/// ```
pub trait Buttonable {
    fn button(
        &mut self,
        text_opts: impl Into<TextOpts>,
        layout: impl Into<LayoutOpts>,
    ) -> EntityCommands;
}

impl<T: Spawn> Buttonable for T {
    fn button(
        &mut self,
        text_opts: impl Into<TextOpts>,
        layout: impl Into<LayoutOpts>,
    ) -> EntityCommands {
        let (text_opts, layout) = (text_opts.into(), layout.into());

        let mut entity = self.spawn((
            Name::new(format!("Button {}", text_opts.text)),
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..layout.node.clone()
            },
            BorderRadius::all(Px(layout.border_radius)),
            BorderColor(layout.border_color),
            BackgroundColor(layout.bg_color),
            InteractionPalette {
                none: NODE_BG,
                hovered: BTN_HOVERED_BG,
                pressed: BTN_PRESSED_BG,
            },
        ));

        entity.label(text_opts, layout);
        entity
    }
}
