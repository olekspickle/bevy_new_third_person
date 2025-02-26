use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::{Screen, player::Player};

const ZOOM: (f32, f32) = (1.5, 30.);
const ZOOM_RADIUS: f32 = (ZOOM.0 + ZOOM.1) / 2.0;
pub const SCENE_EYE: Vec3 = Vec3::new(30., 14., 30.5);
pub const SCENE_TARGET: Vec3 = Vec3::ZERO;

/// Camera logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins(ThirdPersonCameraPlugin)
        .add_systems(OnEnter(Screen::Playing), spawn_scene_camera)
        .add_systems(Update, player_camera.run_if(in_state(Screen::Playing)));
}

#[derive(Component)]
pub struct Ui;

#[derive(Component)]
pub struct SceneCamera;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        Ui,
    ));
}

fn spawn_scene_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        ThirdPersonCamera {
            aim_speed: 3.0,                  // default
            aim_zoom: 0.7,                   // default
            zoom_enabled: true,              // default
            zoom: Zoom::new(ZOOM.0, ZOOM.1), // default
            aim_enabled: true,
            offset_enabled: true,
            offset_toggle_enabled: true,
            gamepad_settings: CustomGamepadSettings::default(),
            ..default()
        },
        SceneCamera,
    );
    commands.spawn(camera);
}

pub fn despawn_scene_camera(mut commands: Commands, query: Query<Entity, With<SceneCamera>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn player_camera(
    mut player: Query<&mut Transform, With<Player>>,
    mut cam: Query<(&mut ThirdPersonCamera, &mut Transform), Without<Player>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let Ok((cam, mut cam_transform)) = cam.get_single_mut() else {
        return;
    };

    // Calculate the desired camera translation based, radius, and xy_offset
    let rotation_matrix = Mat3::from_quat(cam_transform.rotation);

    // apply the offset if offset_enabled is true
    let mut offset = Vec3::ZERO;
    if cam.offset_enabled {
        offset = rotation_matrix.mul_vec3(Vec3::new(cam.offset.offset.0, cam.offset.offset.1, 0.0));
    }

    let desired_translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, ZOOM_RADIUS)) + offset;

    cam_transform.translation = desired_translation + player.translation;
    player.rotation = cam_transform.rotation;
}
