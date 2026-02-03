use super::*;
use bevy::{
    light::{CascadeShadowConfigBuilder, light_consts::lux},
    pbr::{
        Atmosphere, AtmosphereMode, AtmosphereSettings, DistanceFog, FogFalloff, ScatteringMedium,
    },
};
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, sun_cycle.run_if(in_state(Screen::Gameplay)))
        .add_observer(add_skybox_to_camera);
}

markers!(Sun, Moon);

/// Mainly this example:
/// <https://bevyengine.org/examples/3d-rendering/atmosphere/>
pub fn add_skybox_to_camera(
    on: On<Add, SceneCamera>,
    cfg: Res<Config>,
    mut commands: Commands,
    mut scattering_mediums: ResMut<Assets<ScatteringMedium>>,
) -> Result {
    let camera = on.entity;
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: cfg.physics.shadow_distance,
        ..default()
    }
    .build();

    commands.spawn((
        Sun,
        DespawnOnExit(Screen::Gameplay),
        DirectionalLight {
            color: colors::SUN,
            shadows_enabled: true,
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            ..Default::default()
        },
        // Transform::from_translation(Vec3::new(0.0, 0.0, 200.0)),
        cascade_shadow_config.clone(),
    ));

    commands.spawn((
        Moon,
        DespawnOnExit(Screen::Gameplay),
        DirectionalLight {
            color: colors::MOON,
            shadows_enabled: true,
            illuminance: lux::FULL_MOON_NIGHT,
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.0, 10.0, -200.0)),
        cascade_shadow_config,
    ));

    // Lighting
    commands.entity(camera).insert((
        // This is the component that enables atmospheric scattering for a camera
        // TODO: experiment with scattering medium
        Atmosphere::earthlike(scattering_mediums.add(ScatteringMedium::default())),
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings {
            scene_units_to_m: 1.0,
            aerial_view_lut_max_distance: 40_000.0, //  40 km for a vast scene

            // Higher resolution LUTs for smoother gradients and details
            transmittance_lut_size: UVec2::new(512, 256), // Double resolution for smoother light transmission
            sky_view_lut_size: UVec2::new(800, 400),      // Higher resolution for sky appearance
            aerial_view_lut_size: UVec3::new(64, 64, 64), // More detailed aerial perspective

            // Increased sample counts for better accuracy and less artifacts
            transmittance_lut_samples: 60, // More samples for light transmission accuracy
            multiscattering_lut_dirs: 128, // Double directions for multiscattering
            multiscattering_lut_samples: 30, // More samples for multiscattering accuracy
            sky_view_lut_samples: 24,      // More samples for sky appearance
            aerial_view_lut_samples: 15,   // More samples for aerial view depth
            rendering_method: AtmosphereMode::LookupTexture,
            ..Default::default()
        },
    ));

    if cfg.physics.distance_fog {
        commands.entity(camera).insert(distance_fog(cfg));
    }

    Ok(())
}

pub fn distance_fog(cfg: Res<Config>) -> impl Bundle {
    DistanceFog {
        color: Color::srgba(0.35, 0.48, 0.66, 1.0),
        directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
        directional_light_exponent: cfg.physics.fog_directional_light_exponent,
        falloff: FogFalloff::ExponentialSquared { density: 0.002 },
        // falloff: FogFalloff::from_visibility_colors(
        //     cfg.physics.fog_visibility, // distance in world units up to which objects retain visibility (>= 5% contrast)
        //     Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
        //     Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
        // ),
    }
}

#[allow(clippy::type_complexity)]
fn sun_cycle(
    settings: Res<Settings>,
    mut sky_lights: Query<&mut Transform, Or<(With<Moon>, With<Sun>)>>,
    time: Res<Time>,
) {
    match settings.sun_cycle {
        SunCycle::DayNight => sky_lights
            .iter_mut()
            .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * std::f32::consts::PI / 50.0)),
        SunCycle::Nimbus => sky_lights
            .iter_mut()
            .for_each(|mut tf| tf.rotate_y(-time.delta_secs() * std::f32::consts::PI / 50.0)),
    }
}

#[derive(Reflect, Debug, Clone, Serialize, Deserialize)]
pub enum SunCycle {
    DayNight,
    Nimbus,
}

impl SunCycle {
    pub fn as_str(&self) -> &'static str {
        match self {
            SunCycle::DayNight => "DayNight",
            SunCycle::Nimbus => "Nimbus",
        }
    }
}
