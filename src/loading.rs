use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::Screen;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
pub fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Menu)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>(),
    );
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/time-for-fun.ogg")]
    pub bg_play: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    //#[asset(path = "textures/cubemap.png")]
    //pub skybox_image: Handle<Image>,
    //#[asset(path = "textures/cubemap_bc7.ktx2")]
    //pub skybox_bc7: Handle<Image>,
    //#[asset(path = "textures/cubemap_astc4x4.ktx2")]
    //pub skybox_astc: Handle<Image>,
    //#[asset(path = "textures/cubemap_etc2.ktx2")]
    //pub skybox_etc2: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MeshAssets {
    #[asset(path = "models/Player.gltf")]
    pub player: Handle<Mesh>,
}
