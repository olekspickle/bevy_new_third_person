use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::{player::Player, Screen};

const ZOOM: (f32, f32) = (1.5, 30.);
const ZOOM_RADIUS: f32 = (ZOOM.0 + ZOOM.1) / 2.0;

/// Camera logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn)
        .add_systems(
            Update,
            sync_player_with_camera.run_if(in_state(Screen::Playing)),
        );
}

fn spawn(mut commands: Commands) {
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
    );
    commands.spawn(camera);
}

fn sync_player_with_camera(
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
