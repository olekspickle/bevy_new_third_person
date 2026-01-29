use crate::*;

#[cfg(any(feature = "dev_native", not(target_arch = "wasm32")))]
mod dev_tools;
mod music;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        music::plugin,
        #[cfg(any(feature = "dev_native", not(target_arch = "wasm32")))]
        dev_tools::plugin,
    ));
}
