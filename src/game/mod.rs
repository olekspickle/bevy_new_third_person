use crate::*;

#[cfg(feature = "dev_native")]
mod dev_tools;
mod dialogue;
mod mood;

pub use mood::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        mood::plugin,
        #[cfg(feature = "dev_native")]
        dev_tools::plugin,
    ));
}
