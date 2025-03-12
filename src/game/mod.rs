use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod player;
pub mod scene;
pub mod skybox;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        player::plugin,
        scene::plugin,
        skybox::plugin,
    ));
}
