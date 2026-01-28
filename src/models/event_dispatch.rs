use super::*;

pub fn plugin(app: &mut App) {
    app.add_observer(pause)
        .add_observer(mute)
        .add_observer(back);
}

#[derive(Event)]
pub struct GoTo(pub Screen);
#[derive(EntityEvent)]
pub struct Back {
    pub entity: Entity,
    pub screen: Screen,
}
#[derive(EntityEvent)]
pub struct SwitchTab {
    pub entity: Entity,
    pub tab: UiTab,
}
#[derive(EntityEvent)]
pub struct CamCursorToggle(pub Entity);
#[derive(EntityEvent)]
pub struct TogglePause(pub Entity);
#[derive(EntityEvent)]
pub struct ToggleMute(pub Entity);
#[derive(Event)]
pub struct ToggleDebugUi;
#[derive(EntityEvent)]
pub struct ChangeMood {
    pub entity: Entity,
    pub mood: Mood,
}
#[derive(Event)]
pub struct SettingsChanged;

// ================== trigger events on input ========================
fn back(
    on: On<Start<Escape>>,
    state: Res<GameState>,
    screen: Res<State<Screen>>,
    mut commands: Commands,
) {
    match screen.get() {
        Screen::Splash | Screen::Title | Screen::Loading => {}
        _ => {
            let last = state.last_screen.clone();
            commands.trigger(Back {
                entity: on.event_target(),
                screen: last,
            });
        }
    }
}
fn pause(on: On<Start<Pause>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(TogglePause);
}
fn mute(on: On<Start<Mute>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(ToggleMute);
}
