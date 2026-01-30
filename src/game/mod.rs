use crate::*;

#[cfg(all(feature = "dev_native", not(feature = "web")))]
mod dev_tools;
mod mood;

pub use mood::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        mood::plugin,
        #[cfg(all(feature = "dev_native", not(feature = "web")))]
        dev_tools::plugin,
    ));
}
