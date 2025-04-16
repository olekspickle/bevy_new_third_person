use crate::prelude::*;
use bevy::{pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_atmosphere::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(AtmospherePlugin)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(AtmosphereSettings {
            resolution: 128,
            ..default()
        })
        .insert_resource(CycleTimer(Timer::new(
            // Update our atmosphere every 500ms
            bevy::utils::Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .add_systems(OnEnter(Screen::Gameplay), add_skybox_component_to_camera)
        .add_systems(OnExit(Screen::Gameplay), despawn::<AtmosphereCamera>)
        .add_systems(Update, daylight_cycle.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

fn add_skybox_component_to_camera(
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
) {
    let camera = camera.single_mut();
    // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
    commands.entity(camera).insert(AtmosphereCamera::default());
}

// We can edit the Atmosphere resource and it will be updated automatically
fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        // the less the t the longer the cycle
        let t = time.elapsed_secs_wrapped() / 30.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}
