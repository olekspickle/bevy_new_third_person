use bevy::prelude::*;
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_third_person_camera::*;

use crate::Screen;

/// Camera logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), Camera::spawn);
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
            // Marks camera as having a skybox,
            // by default it doesn't specify the render layers the skybox can be seen on
            AtmosphereCamera::default(),
        );
        commands.spawn(camera);
    }
}
