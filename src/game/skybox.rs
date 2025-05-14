use crate::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, light_consts::lux},
    prelude::*,
    render::camera::Exposure,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), add_skybox_to_camera)
        .add_systems(Update, sun_cycle.run_if(in_state(Screen::Gameplay)));
}

#[derive(Component)]
pub struct Sun;

#[derive(Debug)]
pub enum SunCycle {
    DayNight,
    Nimbus,
}

/// Mainly this example:
/// <https://bevyengine.org/examples/3d-rendering/atmosphere/>
fn add_skybox_to_camera(
    cfg: Res<Config>,
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
) -> Result {
    let camera = camera.single_mut()?;

    // Light
    commands.spawn((
        DirectionalLight {
            color: SUN,
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..Default::default()
        },
        Sun,
    ));

    commands.entity(camera).insert((
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        // AtmosphereSettings {
        //     aerial_view_lut_max_distance: 3.2e4,
        //     scene_units_to_m: 1e+4,
        //     ..Default::default()
        // },
        Exposure::SUNLIGHT,
        Tonemapping::BlenderFilmic,
        Bloom::NATURAL,
    ));

    if cfg.physics.fog {
        commands.entity(camera).insert(fog(cfg));
    }

    Ok(())
}

fn sun_cycle(
    settings: Res<Settings>,
    mut suns: Query<&mut Transform, With<DirectionalLight>>,
    time: Res<Time>,
) {
    match settings.sun_cycle {
        SunCycle::DayNight => suns
            .iter_mut()
            .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * std::f32::consts::PI / 50.0)),
        SunCycle::Nimbus => suns
            .iter_mut()
            .for_each(|mut tf| tf.rotate_y(-time.delta_secs() * std::f32::consts::PI / 50.0)),
    }
}

pub fn fog(cfg: Res<Config>) -> impl Bundle {
    // TODO: fog visibility and directional_light_exponent sliders in settings for experimenting
    DistanceFog {
        color: Color::srgba(0.35, 0.48, 0.66, 1.0),
        directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
        directional_light_exponent: cfg.physics.fog_directional_light_exponent,
        falloff: FogFalloff::from_visibility_colors(
            cfg.physics.fog_visibility, // distance in world units up to which objects retain visibility (>= 5% contrast)
            Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
            Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
        ),
    }
}
