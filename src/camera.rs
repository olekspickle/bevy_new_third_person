use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::{actions::Actions, loading::TextureAssets, Screen};

pub struct CameraPlugin;

/// Camera logic is only active during the State `GameState::Playing`
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Playing), Camera::spawn);
    }
}

#[derive(Component)]
pub struct Camera;

impl Camera {
    pub fn spawn(mut commands: Commands) {
        let camera = (
            Camera3d::default(),
            ThirdPersonCamera {
                aim_enabled: true,
                aim_speed: 3.0, // default
                aim_zoom: 0.7,  // default
                offset_enabled: true,
                offset_toggle_enabled: true,
                gamepad_settings: CustomGamepadSettings { ..default() },
                zoom_enabled: true,         // default
                zoom: Zoom::new(1.5, 30.0), // default
                ..default()
            },
        );
        commands.spawn(camera);
    }
}
