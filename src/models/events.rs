use super::*;

pub fn plugin(app: &mut App) {
    app.add_observer(pause).add_observer(mute);
}

#[derive(EntityEvent)]
pub struct ToggleCamCursor(pub Entity);
#[derive(EntityEvent)]
pub struct TogglePause(pub Entity);
#[derive(EntityEvent)]
pub struct ToggleMute(pub Entity);
#[derive(Event)]
pub struct ToggleDebugUi;
#[derive(Event)]
pub struct SettingsChanged;

// ================== trigger events on input ========================
fn pause(on: On<Start<Pause>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(TogglePause);
}
fn mute(on: On<Start<Mute>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(ToggleMute);
}
