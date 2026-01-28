use super::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Modal>()
        .init_resource::<Modals>()
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals);
}

markers!(MenuModal, SettingsModal);

#[derive(Resource, Default, Deref, DerefMut, Debug, Clone)]
pub struct Modals(pub Vec<Modal>);

/// Modal stack. kudo for the idea to @skyemakesgames
/// Only relevant in [`Screen::Gameplay`]
#[derive(States, Reflect, Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
        target.insert(ModalCtx);
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
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
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
    match popped {
        Modal::Main => {
            if let Ok(menu) = menu_marker.single() {
                commands.entity(menu).despawn();
            }
        }
        Modal::Settings => {
            if let Ok(menu) = settings_marker.single() {
                commands.entity(menu).despawn();
            }
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
            .insert(ModalCtx)
            .trigger(TogglePause)
            .trigger(CamCursorToggle);
    }
}

pub fn clear_modals(
    _: On<ClearModals>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
    mut commands: Commands,
    mut modals: ResMut<Modals>,
) {
    for m in &modals.as_deref_mut() {
        match m {
            Modal::Main => {
                if let Ok(modal) = menu_marker.single() {
                    commands.entity(modal).despawn();
                }
            }
            Modal::Settings => {
                if let Ok(modal) = settings_marker.single() {
                    commands.entity(modal).despawn();
                }
            }
        }
    }
}
