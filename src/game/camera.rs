use crate::prelude::*;
use bevy::prelude::*;
use bevy_third_person_camera::*;
use leafwing_input_manager::prelude::ActionState;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_scene_camera)
        .add_systems(OnEnter(Screen::Gameplay), add_third_person_camera)
        .add_systems(Update, change_fov.run_if(in_state(Screen::Gameplay)))
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
        Camera {
            hdr: true,
            ..Default::default()
        },
    ));
}

fn add_third_person_camera(
    cfg: Res<Config>,
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
) -> Result {
    // info!("Camera settings: {:?}", camera2);
    let camera = camera.single_mut()?;
    commands.entity(camera).insert((
        ThirdPersonCamera {
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
        },
        Projection::from(PerspectiveProjection {
            fov: cfg.player.fov.to_radians(),
            ..Default::default()
        }),
    ));

    Ok(())
}

fn change_fov(
    state: Query<&ActionState<Action>>,
    mut settings: ResMut<Settings>,
    mut world_model_projection: Single<&mut Projection>,
) {
    let Ok(state) = state.single() else {
        return;
    };
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        return;
    };

    if state.pressed(&Action::FovIncr) {
        perspective.fov += 1.0_f32.to_radians();
        if perspective.fov.to_degrees() > 160.0 {
            perspective.fov = 20.0_f32.to_radians();
        }
        settings.fov = perspective.fov.to_degrees();
    }
}
