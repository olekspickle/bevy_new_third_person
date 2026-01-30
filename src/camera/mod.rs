use crate::*;
#[cfg(feature = "native")]
use bevy::pbr::ScreenSpaceAmbientOcclusion;
use bevy::{
    anti_alias::taa::TemporalAntiAliasing,
    camera::Exposure,
    core_pipeline::{prepass::DeferredPrepass, tonemapping::Tonemapping},
    light::ShadowFilteringMethod,
    pbr::DefaultOpaqueRendererMethod,
    post_process::bloom::Bloom,
    render::view::Hdr,
};

mod hdr;
#[cfg(feature = "third_person")]
mod third_person;
#[cfg(feature = "top_down")]
mod top_down;

pub fn plugin(app: &mut App) {
    app.insert_resource(DefaultOpaqueRendererMethod::deferred())
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(Screen::Title), add_skybox_to_camera);

    app.add_plugins(hdr::plugin);

    #[cfg(feature = "third_person")]
    app.add_plugins(third_person::plugin);
    #[cfg(feature = "top_down")]
    app.add_plugins(top_down::plugin);
}

/// This enum is converted to an `isize` to be used as a camera's order.
/// Since we have three camera, we use three enum variants.
/// This ordering here mean UI > ViewModel > World.
pub enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        SceneCamera,
        IsDefaultUiCamera,
        Camera {
            order: CameraOrder::World.into(),
            clear_color: Color::srgb_u8(15, 9, 20).into(),
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(100., 50., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        Exposure::BLENDER,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        Hdr,
        (
            Msaa::Off,
            TemporalAntiAliasing::default(),
            ShadowFilteringMethod::Temporal,
            DeferredPrepass,
        ),
        // See https://github.com/bPluginevyengine/bevy/issues/20459
        #[cfg(feature = "native")]
        ScreenSpaceAmbientOcclusion::default(),
    ));
}
