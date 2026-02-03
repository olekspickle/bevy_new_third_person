use crate::*;
#[cfg(feature = "native")]
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::{
    anti_alias::taa::TemporalAntiAliasing, camera::Exposure,
    core_pipeline::tonemapping::Tonemapping, light::ShadowFilteringMethod,
    post_process::bloom::Bloom, render::view::Hdr,
};

mod hdr;
#[cfg(feature = "third_person")]
mod third_person;
#[cfg(feature = "top_down")]
mod top_down;

pub fn plugin(app: &mut App) {
    app.add_plugins(hdr::plugin)
        .add_systems(Startup, spawn_camera);

    #[cfg(feature = "third_person")]
    app.add_plugins(third_person::plugin);
    #[cfg(feature = "top_down")]
    app.add_plugins(top_down::plugin);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        SceneCamera,
        IsDefaultUiCamera,
        Camera::default(),
        Camera3d::default(),
        Transform::from_xyz(100., 50., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        (
            Exposure::BLENDER,
            Tonemapping::TonyMcMapface,
            Bloom::NATURAL,
            Hdr,
        ),
        // performance critical
        (
            Msaa::Off,
            #[cfg(not(feature = "web"))] // breaks wasm
            TemporalAntiAliasing::default(),
            ShadowFilteringMethod::Temporal,
            #[cfg(feature = "native")] // See https://github.com/bPluginevyengine/bevy/issues/20459
            ScreenSpaceAmbientOcclusion::default(),
        ),
    ));
}
