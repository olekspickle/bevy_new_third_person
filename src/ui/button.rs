use super::*;

#[derive(Debug, Clone)]
pub struct ButtonOpts {
    pub font: TextFont,
    pub label_opts: LabelOpts,
    pub bg_color: Color,
    pub color: Color,
    pub node: Node,
}

impl ButtonOpts {
    pub fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label_opts = LabelOpts::from(label.into());
        self
    }
    pub fn with_node(mut self, n: Node) -> Self {
        self.node = n;
        self
    }
}

// For something like "my-label".into()
impl<T: Into<String>> From<T> for ButtonOpts {
    fn from(value: T) -> Self {
        Self {
            label_opts: LabelOpts::from(value),
            ..default()
        }
    }
}

impl Default for ButtonOpts {
    fn default() -> Self {
        Self {
            color: COLOR_NORM,
            bg_color: BG_COLOR_NORM,
            node: Node {
                width: Px(150.0),
                height: Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Px(1.0)),
                ..Default::default()
            },
            label_opts: "button".into(),
            font: TextFont::from_font_size(FONT_SIZE),
        }
    }
}

/// # Example
/// ```rust,no_run
///
/// fn setup_button(commands: &mut Commands, font: ) {
///   .with_child((
///       Text::new("Button"),
///       TextFont {
///           font: asset_server.load("fonts/FiraSans-Regular.ttf"),
///           font_size: 33.0,
///           ..default()
///       },
///       TextColor(Color::srgb(0.9, 0.9, 0.9)),
///   ));
/// ```
pub trait Buttonable {
    fn button(&mut self, opts: &ButtonOpts) -> EntityCommands;
}

impl<T: Spawn> Buttonable for T {
    fn button(&mut self, opts: &ButtonOpts) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new(format!("Button {}", opts.label_opts.text)),
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..opts.node.clone()
            },
            BorderRadius::MAX,
            BorderColor(Color::BLACK),
            BackgroundColor(COLOR_NORM),
        ));

        entity.label(&opts.into());
        entity
    }
}

//fn button_system(
//    mut interaction_query: Query<
//        (
//            &Interaction,
//            &mut BackgroundColor,
//            &mut BorderColor,
//            &Children,
//        ),
//        (Changed<Interaction>, With<Button>),
//    >,
//    mut text_query: Query<&mut Text>,
//) {
//    use bevy::color::palettes::basic::*;
//    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
//        let mut text = text_query.get_mut(children[0]).unwrap();
//        match *interaction {
//            Interaction::Pressed => {
//                **text = "Press".to_string();
//                *color = BG_COLOR_PRESSED.into();
//                border_color.0 = RED.into();
//            }
//            Interaction::Hovered => {
//                **text = "Hover".to_string();
//                *color = BG_COLOR_HOVER.into();
//                border_color.0 = Color::WHITE;
//            }
//            Interaction::None => {
//                **text = "Button".to_string();
//                *color = COLOR_NORM.into();
//                border_color.0 = Color::BLACK;
//            }
//        }
//    }
//}
