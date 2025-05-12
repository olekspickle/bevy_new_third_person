use bevy::prelude::{App, Resource};

pub mod camera;
pub mod player;
pub mod scene;
pub mod settings;
pub mod skybox;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        scene::plugin,
        player::plugin,
        skybox::plugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
