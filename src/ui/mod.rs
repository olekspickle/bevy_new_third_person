use crate::*;
use bevy::{
    ecs::{
        spawn::SpawnRelated,
        system::{Commands, Query},
    },
    reflect::Reflect,
    ui::{
        AlignItems, BorderRadius, Display, FlexDirection, JustifyContent, Node, PositionType,
        UiRect, Val::*,
    },
    ui_widgets::Button,
    window::Window,
};
use serde::{Deserialize, Serialize};

mod constants;
mod interaction;
pub mod modal;
#[cfg(all(feature = "dev", not(feature = "web")))]
mod perf;
mod prefabs;
mod props;
mod widget;

pub use constants::*;
pub use modal::*;
pub use prefabs::*;
pub use props::*;
pub use widget::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((prefabs::plugin, interaction::plugin));

    #[cfg(all(feature = "dev", not(feature = "web")))]
    app.add_plugins(perf::plugin);
}
