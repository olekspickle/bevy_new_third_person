//! Helper traits for creating common widgets.
//!
use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, ui::Val::*};

mod button;
mod container;
mod label;
mod opts;

pub use button::*;
pub use container::*;
pub use label::*;
pub use opts::*;

/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
pub trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        ChildBuild::spawn(self, bundle)
    }
}

impl Spawn for EntityCommands<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.insert(bundle).reborrow()
    }
}
