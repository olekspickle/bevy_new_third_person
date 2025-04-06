use crate::prelude::*;
use bevy::prelude::*;
use bevy_third_person_camera::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene_camera)
        .add_systems(OnEnter(Screen::Gameplay), add_third_person_camera)
        .add_systems(OnExit(Screen::Gameplay), despawn::<ThirdPersonCamera>);
}

#[derive(Component)]
pub struct SceneCamera;

pub fn spawn_scene_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        SceneCamera,
        IsDefaultUiCamera,
        Transform::from_xyz(100., 50., 100.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn add_third_person_camera(
    cfg: Res<Config>,
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
) {
    let camera = camera.single_mut();
    commands.entity(camera).insert(ThirdPersonCamera {
        aim_speed: 3.0,     // default
        aim_zoom: 0.7,      // default
        zoom_enabled: true, // default
        zoom: Zoom::new(cfg.player.zoom.0, cfg.player.zoom.1),
        aim_enabled: true,
        offset_enabled: true,
        offset_toggle_enabled: true,
        cursor_lock_key: KeyCode::KeyL,
        gamepad_settings: CustomGamepadSettings::default(),
        ..default()
    });
}
