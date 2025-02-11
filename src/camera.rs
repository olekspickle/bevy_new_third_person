use bevy::prelude::*;
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
                aim_speed: 3.0,             // default
                aim_zoom: 0.7,              // default
                zoom_enabled: true,         // default
                zoom: Zoom::new(1.5, 30.0), // default
                aim_enabled: true,
                offset_enabled: true,
                offset_toggle_enabled: true,
                gamepad_settings: CustomGamepadSettings::default(),
                ..default()
            },
        );
        commands.spawn(camera);
    }
}
