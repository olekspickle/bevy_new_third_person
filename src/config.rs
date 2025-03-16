use crate::prelude::*;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]));
    app.load_resource_from_path::<Config>("config.ron");
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Geometry {
    pub quantity: u32,
    pub y_upper_bound: f32,
    pub y_lower_bound: f32,
    pub x_upper_bound: f32,
    pub x_lower_bound: f32,
    pub z_upper_bound: f32,
    pub z_lower_bound: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    pub scale: f32,
    pub geometry: Geometry,
}

#[derive(Asset, Clone, Reflect, Resource)]
pub struct Textures {
    #[dependency]
    pub bevy: Handle<Image>,
    #[dependency]
    pub github: Handle<Image>,
}

#[derive(Asset, Clone, Reflect, Resource)]
pub struct Models {
    #[dependency]
    pub player: Handle<Gltf>,
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

impl FromWorld for Models {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("models/player.glb"),
        }
    }
}
