use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene_camera);
}

#[derive(Component)]
pub struct SceneCamera;

pub fn spawn_scene_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        SceneCamera,
        IsDefaultUiCamera,
    ));
}
