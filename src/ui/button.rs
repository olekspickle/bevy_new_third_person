use super::*;
use bevy::color::palettes::basic::*;

#[derive(Debug, Clone)]
pub struct ButtonOpts {
    pub label: String,
    pub color: Color,
    pub bg_color: Color,
    pub width: f32,
    pub height: f32,
    pub border: f32,
    pub font: TextFont,
}

impl ButtonOpts {
    pub fn with_font(mut self, font: TextFont) -> Self {
        self.font = font;
        self
    }
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
}

// For something like "my-label".into()
impl<T: Into<String>> From<T> for ButtonOpts {
    fn from(value: T) -> Self {
        Self {
            label: value.into(),
            ..default()
        }
    }
}

impl Default for ButtonOpts {
    fn default() -> Self {
        Self {
            color: COLOR_NORM,
            bg_color: BG_COLOR_NORM,
            width: 150.0,
            height: 60.0,
            border: 5.0,
            label: "button".into(),
            font: TextFont {
                font_size: 24.0,
                ..default()
            },
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
            Name::new(format!("Button {}", opts.label)),
            Button,
            Node {
                width: Val::Px(opts.width),
                height: Val::Px(opts.height),
                border: UiRect::all(Val::Px(opts.border)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::MAX,
            BorderColor(Color::BLACK),
            BackgroundColor(COLOR_NORM),
        ));

        entity.label(&opts.into());
        entity
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, button_system);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            ChildBuild::spawn(
                parent,
                (
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(COLOR_NORM),
                ),
            )
            .with_child((
                Text::new("Button"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = BG_COLOR_PRESSED.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = BG_COLOR_HOVER.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = COLOR_NORM.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
