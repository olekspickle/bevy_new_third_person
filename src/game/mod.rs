use crate::*;

#[cfg(feature = "dev")]
mod dev_tools;
mod mood;

pub use mood::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        mood::plugin,
        #[cfg(feature = "dev")]
        dev_tools::plugin,
    ));
}
