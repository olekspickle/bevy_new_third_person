use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod player;
pub mod scene;
pub mod settings;
pub mod skybox;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        audio::plugin,
        settings::plugin,
        camera::plugin,
        player::plugin,
        scene::plugin,
        skybox::plugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
