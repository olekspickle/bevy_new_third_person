use super::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<GameState>().register_type::<Mood>();
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct GameState {
    pub last_screen: Screen,
    pub current_mood: Mood,

    pub diagnostics: bool,
    pub debug_ui: bool,
    pub paused: bool,
    pub muted: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            last_screen: Screen::Title,
            current_mood: Mood::Exploration,
            diagnostics: true,
            debug_ui: true,
            paused: false,
            muted: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        self.paused = false;
        self.muted = false;
    }
}

/// The game's main screen states.
/// See <https://bevy-cheatbook.github.io/programming/states.html>
/// Or <https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs>
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
pub enum Screen {
    // TODO: splash should be first
    // Bevy tribute <3
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Tutorial,
    Credits,
    Settings,
    // Here the menu is drawn and waiting for player interaction
    Title,
    // During this State the actual game logic is executed
    Gameplay,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[reflect(Component)]
pub enum Mood {
    #[default]
    Exploration,
    Combat,
}
