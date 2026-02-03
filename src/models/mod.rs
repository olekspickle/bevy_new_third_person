use crate::*;
use bevy_enhanced_input::prelude::*;
use serde::Serialize;

mod config;
mod event_dispatch;
mod ext_traits;
mod input;
mod keybinding;
mod primitives;
mod settings;
mod states;

pub use config::*;
pub use event_dispatch::*;
pub use ext_traits::*;
pub use input::*;
pub use keybinding::*;
pub use primitives::*;
pub use settings::*;
pub use states::*;

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (
            AppSystems::UserInput,
            AppSystems::TickTimers,
            AppSystems::ChangeUi,
            AppSystems::PlaySounds,
            AppSystems::PlayAnimations,
            AppSystems::Update,
        )
            .chain(),
    );

    app.add_plugins((settings::plugin, states::plugin, event_dispatch::plugin));
}

/// High-level groupings of systems for the app in the [`Update`] schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
/// courtesy of janhohenheim
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// User Input
    UserInput,
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
