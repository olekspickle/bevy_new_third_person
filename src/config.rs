use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{Screen, asset_tracking::LoadResource};

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]));
    app.load_resource_from_path::<Config>("config.ron");
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    scale: f32,
    geometry: Vec<(f32, f32, f32)>,
}

//#[derive(AssetCollection, Resource)]
//pub struct AudioAssets {
//    #[asset(path = "audio/time-for-fun.ogg")]
//    pub bg_play: Handle<AudioSource>,
//}
//#[derive(AssetCollection, Resource)]
//pub struct TextureAssets {
//    #[asset(path = "images/bevy.png")]
//    pub bevy: Handle<Image>,
//    #[asset(path = "images/github.png")]
//    pub github: Handle<Image>,
//}
//#[derive(AssetCollection, Resource)]
//pub struct MeshAssets {
//    #[asset(path = "models/Player.gltf")]
//    pub player: Handle<Mesh>,
//}
//
//font: asset_server.load("fonts/FiraSans-Regular.ttf"),
//#[derive(AssetCollection, Resource)]
//pub struct ConfigAsset {
//    #[asset(path = "config.ron")]
//    pub config: Handle<Config>,
//}
