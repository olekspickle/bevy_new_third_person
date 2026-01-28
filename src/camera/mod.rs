use crate::*;
use avian3d::prelude::*;
use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::Exposure,
    core_pipeline::{prepass::DeferredPrepass, tonemapping::Tonemapping},
    pbr::DefaultOpaqueRendererMethod,
    post_process::bloom::Bloom,
    render::view::Hdr,
};

#[cfg(feature = "third_person")]
mod third_person;
#[cfg(feature = "top_down")]
mod top_down;

pub fn plugin(app: &mut App) {
    app.insert_resource(DefaultOpaqueRendererMethod::deferred())
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(Screen::Title), add_skybox_to_camera);

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
        Tonemapping::BlenderFilmic,
        Exposure::OVERCAST,
        Bloom::NATURAL,
        Hdr,
        Msaa::Off,
        DeferredPrepass,
        TemporalAntiAliasing::default(),
    ));
}
