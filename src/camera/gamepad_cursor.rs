//! Helper module that enables using gamepad to simulate mouse and keyboard input.
use super::*;
use bevy::{
    input::{InputSystems, gamepad::gamepad_event_processing_system, keyboard::Key},
    window::{PrimaryWindow, WindowEvent},
};
use std::collections::HashMap;

pub fn plugin(app: &mut App) {
    app.init_resource::<GamepadCursorSettings>();
    app.register_required_components::<Gamepad, GamepadCursor>();
    app.register_required_components::<Gamepad, GamepadMappings>();

    app.add_systems(
        FixedUpdate,
        (parse_gamepad_axis_events, gamepad_update_mouse_pos)
            .chain()
            .run_if((any_with_component::<PrimaryWindow>).and(any_with_component::<GamepadCursor>)),
    );
    app.add_systems(
        PreUpdate,
        (gamepad_buttons_to_window_events.after(gamepad_event_processing_system))
            .in_set(InputSystems)
            .run_if(
                (any_with_component::<PrimaryWindow>).and(any_with_component::<GamepadMappings>),
            ),
    );
}

/// Gamepad Cursor component that allows moving cursor inside window by using gamepad axis input
#[derive(Component, Reflect, Debug)]
#[reflect(Default)]
#[require(Gamepad)]
pub struct GamepadCursorMappings {
    /// The gamepad axis used for horizontal cursor movement input.
    pub x: GamepadAxis,
    /// The gamepad axis used for vertical cursor movement input.
    pub y: GamepadAxis,
}
/// Gamepad Cursor component that allows moving cursor inside window by using gamepad axis input
#[derive(Component, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Default)]
#[require(GamepadCursorMappings)]
pub struct GamepadCursor(pub Vec2);

/// Mapping of gamepad buttons to keyboard or mouse buttons.
#[derive(Component, Reflect, Debug, Deref, DerefMut)]
#[reflect(Default)]
#[require(Gamepad)]
pub struct GamepadMappings(pub HashMap<GamepadButton, Mapping>);

/// Enum representing mappings from a gamepad button to either a keyboard or mouse button.
#[derive(Debug, PartialEq, Reflect)]
pub enum Mapping {
    /// Maps a gamepad button to a keyboard key.
    Keyboard(KeyCode, Key),
    /// Maps a gamepad button to a mouse button.
    Mouse(MouseButton),
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct GamepadCursorSettings {
    /// Margins to prevent the cursor from moving out of the window bounds.
    pub margins: Vec2,
    /// Multiplier applied to each axis for cursor movement scaling.
    pub cursor_move_multiplier: Vec2,
}

impl GamepadCursorSettings {
    /// Margins to prevent the cursor from moving out of the window bounds.
    pub fn clamp_mouse_pos(
        &self,
        resolution: &bevy::window::WindowResolution,
        proposed_pos: Vec2,
    ) -> Option<Vec2> {
        Some(
            proposed_pos
                .max(self.margins)
                .min(resolution.size() - self.margins),
        )
    }
}

impl GamepadMappings {
    /// Searches the mappings if event is matching them, if so makes a WindowEvent based on the [`bevy::input::gamepad::GamepadButtonStateChangedEvent`].
    pub fn convert_event(
        &self,
        gamepad_event: &bevy::input::gamepad::GamepadButtonStateChangedEvent,
        window: Entity,
    ) -> Option<WindowEvent> {
        let mapping = self.get(&gamepad_event.button)?;

        let event = match mapping {
            Mapping::Keyboard(key_code, key) => {
                let ev = bevy::input::keyboard::KeyboardInput {
                    key_code: *key_code,
                    logical_key: key.clone(),
                    window,
                    state: gamepad_event.state,
                    repeat: false,
                    text: None,
                };
                WindowEvent::KeyboardInput(ev)
            }
            Mapping::Mouse(mouse_button) => {
                let ev = bevy::input::mouse::MouseButtonInput {
                    button: *mouse_button,
                    state: gamepad_event.state,
                    window,
                };
                WindowEvent::MouseButtonInput(ev)
            }
        };
        Some(event)
    }
}

impl Default for GamepadCursorSettings {
    fn default() -> Self {
        Self {
            margins: Vec2::new(10.0, 10.0),
            cursor_move_multiplier: Vec2::new(3.0, -3.0),
        }
    }
}

impl Default for GamepadCursorMappings {
    fn default() -> Self {
        Self {
            x: GamepadAxis::RightStickX,
            y: GamepadAxis::RightStickY,
        }
    }
}

impl Default for GamepadMappings {
    fn default() -> Self {
        Self(
            [
                (
                    GamepadButton::RightTrigger2,
                    Mapping::Mouse(MouseButton::Left),
                ),
                (
                    GamepadButton::LeftTrigger2,
                    Mapping::Mouse(MouseButton::Right),
                ),
                (
                    GamepadButton::Start,
                    Mapping::Keyboard(KeyCode::Escape, Key::Escape),
                ),
            ]
            .into(),
        )
    }
}

fn parse_gamepad_axis_events(
    mut axis_events: MessageReader<bevy::input::gamepad::GamepadAxisChangedEvent>,
    mut gamepad_cursors: Query<(&GamepadCursorMappings, &mut GamepadCursor)>,
    settings: Res<GamepadCursorSettings>,
) {
    for axis_event in axis_events.read() {
        let Ok((mappings, mut cursor)) = gamepad_cursors.get_mut(axis_event.entity) else {
            continue;
        };

        match axis_event.axis {
            axis if axis == mappings.x => {
                cursor.x = axis_event.value * settings.cursor_move_multiplier.x
            }
            axis if axis == mappings.y => {
                cursor.y = axis_event.value * settings.cursor_move_multiplier.y
            }
            _ => {}
        }
    }
}

fn gamepad_update_mouse_pos(
    gamepad_cursors: Query<&GamepadCursor>,
    settings: Res<GamepadCursorSettings>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    let intent = gamepad_cursors.iter().fold(Vec2::ZERO, |acc, x| acc + **x);

    if intent.length_squared() < 0.1 {
        return;
    }
    let Some(cursor_pos) = window.cursor_position() else {
        let center = Some(window.resolution.size() / 2.0);
        window.set_cursor_position(center);
        return;
    };
    let final_pos = settings.clamp_mouse_pos(&window.resolution, cursor_pos + intent);
    window.set_cursor_position(final_pos);
}

fn gamepad_buttons_to_window_events(
    mut button_event: MessageReader<bevy::input::gamepad::GamepadButtonStateChangedEvent>,
    mut writer: MessageWriter<WindowEvent>,
    window_entity: Single<Entity, With<PrimaryWindow>>,
    mappings: Query<&GamepadMappings>,
) {
    for ev in button_event.read() {
        if let Ok(Some(event)) = mappings
            .get(ev.entity)
            .map(|mapping| mapping.convert_event(ev, *window_entity))
        {
            writer.write(event);
        };
    }
}
