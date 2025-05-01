use crate::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, light_consts::lux},
    prelude::*,
    render::camera::Exposure,
};

pub fn plugin(app: &mut App) {
    // app.insert_resource(CycleTimer(Timer::new(
    //     // Update our atmosphere every 500ms
    //     utils::Duration::from_millis(500),
    //     TimerMode::Repeating,
    // )))
    app.add_systems(OnEnter(Screen::Gameplay), add_skybox_to_camera)
        .add_systems(Update, daylight_cycle.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct Sun;

// #[derive(Resource)]
// struct CycleTimer(Timer);

fn add_skybox_to_camera(
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
) -> Result {
    let camera = camera.single_mut()?;

    // Light
    commands.spawn((
        DirectionalLight {
            color: SUN,
            shadows_enabled: true,
            // lux::RAW_SUNLIGHT is recommended for use with this feature, since
            // other values approximate sunlight *post-scattering* in various
            // conditions. RAW_SUNLIGHT in comparison is the illuminance of the
            // sun unfiltered by the atmosphere, so it is the proper input for
            // sunlight to be filtered by the atmosphere.
            illuminance: lux::RAW_SUNLIGHT,
            ..Default::default()
        },
        Sun,
    ));

    // Marks camera as having a skybox, by default it doesn't specify the render layers the skybox can be seen on
    commands.entity(camera).insert((
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings {
            aerial_view_lut_max_distance: 3.2e5,
            scene_units_to_m: 1e+4,
            ..Default::default()
        },
        // The directional light illuminance  used in this scene
        // (the one recommended for use with this feature) is
        // quite bright, so raising the exposure compensation helps
        // bring the scene to a nicer brightness range.
        Exposure::SUNLIGHT,
        // Tonemapper chosen just because it looked good with the scene, any
        // tonemapper would be fine :)
        Tonemapping::AcesFitted,
        // Bloom gives the sun a much more natural look.
        Bloom::NATURAL,
        DistanceFog {
            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                15.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));

    Ok(())
}

fn daylight_cycle(mut suns: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
    suns.iter_mut()
        .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * std::f32::consts::PI / 10.0));
}

// // We can edit the Atmosphere resource and it will be updated automatically
// fn daylight_cycle(
//     mut atmosphere: AtmosphereMut<Nishita>,
//     mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
//     mut timer: ResMut<CycleTimer>,
//     time: Res<Time>,
// ) {
//     timer.0.tick(time.delta());
//
//     if timer.0.finished() {
//         // the less the t the longer the cycle
//         let t = time.elapsed_secs_wrapped() / 30.0;
//         atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());
//
//         if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
//             light_trans.rotation = Quat::from_rotation_x(-t);
//             directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
//         }
//     }
// }
