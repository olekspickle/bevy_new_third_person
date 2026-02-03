use super::*;

pub fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInput>()
        .add_observer(on_player_add);
}

#[derive(Component, Default)]
pub(crate) struct PlayerInput;

fn on_player_add(on: On<Add, Player>, mut commands: Commands) {
    commands.entity(on.entity).insert(actions!(PlayerInput[
        (
            Action::<Movement>::new(),
            DeadZone::default(),
            Scale::splat(0.3),
            Bindings::spawn(( Cardinal::wasd_keys(), Cardinal::arrows(), Axial::left_stick() )),
        ),
        (
            Action::<Crouch>::new(),
            bindings![KeyCode::ControlLeft, GamepadButton::East],
        ),
        (
            Action::<Jump>::new(),
            bindings![KeyCode::Space, GamepadButton::South],
        ),
        (
            Action::<Dash>::new(),
            bindings![KeyCode::AltLeft, GamepadButton::LeftTrigger],
        ),
        (
            Action::<Sprint>::new(),
            bindings![KeyCode::ShiftLeft, GamepadButton::LeftThumb],
        ),
        (
            Action::<Attack>::new(),
            bindings![MouseButton::Left, GamepadButton::RightTrigger2],
        ),

        (
            Action::<Pause>::new(),
            bindings![KeyCode::KeyP],
        ),
        (
            Action::<Mute>::new(),
            bindings![KeyCode::KeyM],
        ),
        (
            Action::<Escape>::new(),
            ActionSettings {
                require_reset: true,
                ..Default::default()
            },
            bindings![KeyCode::Escape, GamepadButton::Select],
        ),
    ]));
}
