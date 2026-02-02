use super::*;
use bevy_enhanced_input::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Modal>()
        .add_input_context::<ModalInput>()
        .init_resource::<Modals>()
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals)
        .add_observer(on_modal_add);
}

fn spawn_ctx(mut commands: Commands) {
    commands.spawn(ModalInput);
}

#[derive(Component, Default)]
pub(crate) struct ModalInput;

fn on_modal_add(on: On<Add, Modal>, mut commands: Commands) {
    commands.entity(on.entity).insert((
        ContextPriority::<ModalInput>::new(1),
        actions!(ModalInput[
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

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone)]
pub struct Modals(pub Vec<Modal>);

/// Modal stack. kudo for the idea to @skyemakesgames
/// Only relevant in [`Screen::Gameplay`]
#[derive(States, Component, Reflect, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modal {
    #[default]
    Main,
    Settings,
}

#[derive(EntityEvent)]
pub struct NewModal {
    pub entity: Entity,
    pub modal: Modal,
}
#[derive(EntityEvent)]
pub struct PopModal(pub Entity);

#[derive(EntityEvent)]
pub struct ClearModals(pub Entity);

pub fn click_pop_modal(on: On<Pointer<Click>>, mut commands: Commands) {
    commands.entity(on.entity).trigger(PopModal);
}

pub fn add_new_modal(
    on: On<NewModal>,
    screen: Res<State<Screen>>,
    mut commands: Commands,
    mut modals: ResMut<Modals>,
    state: Res<GameState>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }

    let mut target = commands.entity(on.entity);
    if modals.is_empty() {
        target.insert(ModalInput);
        if Modal::Main == on.modal && !state.paused {
            target.trigger(TogglePause);
        }
        target.trigger(CamCursorToggle);
    }

    // despawn all previous modal entities to avoid clattering
    target.trigger(ClearModals);
    match on.event().modal {
        Modal::Main => commands.spawn(menu_modal()),
        Modal::Settings => commands.spawn(settings_modal()),
    };

    modals.push(on.event().modal.clone());
}

pub fn pop_modal(
    pop: On<PopModal>,
    screen: Res<State<Screen>>,
    modals_q: Query<(Entity, &Modal)>,
    mut commands: Commands,
    mut modals: ResMut<Modals>,
) {
    if Screen::Gameplay != *screen.get() {
        return;
    }

    debug!("Chat, are we popping? {:?}", modals);
    // just a precaution
    assert!(!modals.is_empty());

    let popped = modals
        .pop()
        .expect("failed to pop modal after assert on non-empty passed");

    for (e, modal) in modals_q.iter() {
        if *modal == popped {
            commands.entity(e).despawn();
        }
    }

    // respawn next in the modal stack
    if let Some(modal) = modals.last() {
        match modal {
            Modal::Main => commands.spawn(menu_modal()),
            Modal::Settings => commands.spawn(settings_modal()),
        };
    }

    if modals.is_empty() {
        info!("PopModal target entity: {}", pop.event_target());
        commands
            .entity(pop.event_target())
            .insert(ModalInput)
            .trigger(TogglePause)
            .trigger(CamCursorToggle);
    }
}

pub fn clear_modals(
    _: On<ClearModals>,
    modals_q: Query<Entity, With<Modal>>,
    mut commands: Commands,
) {
    for m in modals_q.iter() {
        commands.entity(m).despawn();
    }
}
