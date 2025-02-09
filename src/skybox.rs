use bevy::{pbr::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_atmosphere::prelude::*;

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

pub fn plugin(app: &mut App) {
    // Default Atmosphere material, we can edit it to simulate another planet
    app.insert_resource(AtmosphereModel::default())
        .insert_resource(AtmosphereSettings {
            resolution: 24,
            ..default()
        })
        // Update our atmosphere every 50ms
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(50),
            TimerMode::Repeating,
        )));
    //.add_systems(Update, daylight_cycle.run_if(in_state(Screen::Playing)));
}

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_secs_wrapped() / 7.0;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
        }
    }
}
