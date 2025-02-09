use bevy::{
    core_pipeline::Skybox,
    image::CompressedImageFormats,
    prelude::*,
    render::{
        render_resource::{TextureViewDescriptor, TextureViewDimension},
        renderer::RenderDevice,
    },
};
use std::f32::consts::PI;

use crate::{loading::TextureAssets, scene, Screen};

#[derive(Resource)]
pub struct Cubemap {
    pub is_loaded: bool,
    pub index: usize,
    pub image_handle: Handle<Image>,
}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            cycle_cubemap_asset.after(scene::setup),
            asset_loaded.after(cycle_cubemap_asset),
            animate_light_direction,
        ),
    )
    .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 1.0,
    });
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: Default::default(),
    });
}

fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    textures: Res<TextureAssets>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_secs();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let cubemaps = &[
        (&textures.skybox_image, CompressedImageFormats::NONE),
        (&textures.skybox_astc, CompressedImageFormats::ASTC_LDR),
        (&textures.skybox_bc7, CompressedImageFormats::BC),
        (&textures.skybox_etc2, CompressedImageFormats::ETC2),
    ];

    let mut new_index = cubemap.index;
    for _ in 0..cubemaps.len() {
        new_index = (new_index + 1) % cubemaps.len();
        if supported_compressed_formats.contains(cubemaps[new_index].1) {
            break;
        }
        info!(
            "Skipping format which is not supported by current hardware: {:?}",
            cubemaps[new_index]
        );
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = cubemaps[cubemap.index].0.clone();
    cubemap.is_loaded = false;
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
    textures: Res<TextureAssets>,
) {
    let cubemaps = &[
        &textures.skybox_image,
        &textures.skybox_astc,
        &textures.skybox_bc7,
        &textures.skybox_etc2,
    ];

    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle).is_loaded() {
        println!(
            "Swapping to {}...",
            cubemaps[cubemap.index]
                .path()
                .map(ToString::to_string)
                .unwrap_or_default()
        );
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
