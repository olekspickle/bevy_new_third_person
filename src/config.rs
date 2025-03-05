use crate::prelude::*;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]));
    app.load_resource_from_path::<Config>("config.ron");
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    scale: f32,
    geometry: Vec<(f32, f32, f32)>,
}

#[derive(Asset, Clone, Reflect, Resource)]
pub struct Textures {
    #[dependency]
    pub bevy: Handle<Image>,
    #[dependency]
    pub github: Handle<Image>,
}
#[derive(Asset, Clone, Reflect, Resource)]
pub struct Meshes {
    #[dependency]
    pub player: Handle<Mesh>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bevy: assets.load("images/bevy.png"),
            github: assets.load("images/github.png"),
        }
    }
}

impl FromWorld for Meshes {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("models/Player.gltf"),
        }
    }
}
