use super::*;
use bevy_ahoy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(EnhancedInputPlugin)
        .add_input_context::<PlayerCtx>()
        .add_input_context::<ModalCtx>()
        .add_systems(Startup, spawn_ctx)
        .add_observer(add_modal_ctx)
        .add_observer(add_player_ctx);
}

markers!(PlayerCtx, ModalCtx);

fn spawn_ctx(mut commands: Commands) {
    commands.spawn(ModalCtx);
}

// #[derive(InputAction)]
// #[action_output(Vec2)]
// pub struct Movement;
//
// #[derive(InputAction)]
// #[action_output(bool)]
// pub struct Jump;
//
// #[derive(InputAction)]
// #[action_output(bool)]
// pub struct Crouch;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Attack;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Sprint;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dash;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Mute;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Escape;

#[derive(InputAction)]
#[action_output(Vec2)]
struct NavigateModal;

/// Controller element select. F.e. for inventory cell
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Select;

/// Controller tab switch right
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct RightTab;

/// Controller tab switch left
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct LeftTab;

pub fn add_player_ctx(add: On<Add, PlayerCtx>, mut commands: Commands) {
    let mut e = commands.entity(add.entity);

    e.insert(actions!(PlayerCtx[
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

fn add_modal_ctx(add: On<Add, ModalCtx>, mut commands: Commands) {
    commands.entity(add.entity).insert((
        ContextPriority::<ModalCtx>::new(1),
        actions!(ModalCtx[
            (
                Action::<NavigateModal>::new(),
                ActionSettings {
                    require_reset: true,
                    ..Default::default()
                },
                Bindings::spawn((
                    Spawn((Binding::mouse_motion(),Scale::splat(0.1), Negate::all())),
                    Axial::right_stick().with((Scale::splat(2.0), Negate::x())) ,
                )),
            ),
        (
            Action::<Select>::new(),
            bindings![KeyCode::Enter, GamepadButton::South, MouseButton::Left],
        ),
        (
            Action::<RightTab>::new(),
            bindings![KeyCode::Tab, GamepadButton::RightTrigger],
        ),
        (
            Action::<LeftTab>::new(),
            bindings![GamepadButton::LeftTrigger],
        ),
        (
            Action::<Escape>::new(),
                ActionSettings {
                    require_reset: true,
                    ..Default::default()
                },
            bindings![KeyCode::Escape, GamepadButton::Select],
        ),
        ]),
    ));
}
