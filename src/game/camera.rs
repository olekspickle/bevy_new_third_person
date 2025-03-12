use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene_camera);
}

#[derive(Component)]
pub struct SceneCamera;

pub fn spawn_scene_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        //Camera {
        //    order: 0,
        //    ..default()
        //},
        Msaa::Sample4,
        SceneCamera,
        IsDefaultUiCamera,
    );
    commands.spawn(camera);
}
