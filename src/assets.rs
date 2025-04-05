use crate::prelude::*;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_seedling::sample::Sample;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]));
    app.load_resource_from_path::<Config>("config.ron");
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Geometry {
    pub main_plane: f32,
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct Hitbox {
    pub radius: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Movement {
    pub dash_distance: f32,
    pub speed: f32,
    pub actions_in_air: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect)]
pub struct PlayerConfig {
    pub movement: Movement,
    pub hitbox: Hitbox,
}

#[derive(Clone, Debug, Serialize, Deserialize, Reflect, Asset, Resource)]
pub struct Config {
    pub scale: f32,
    pub zoom: (f32, f32),
    pub geometry: Geometry,
    pub player: PlayerConfig,
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

#[derive(Resource, Asset, Reflect, Clone)]
pub struct AudioSources {
    // SFX
    #[dependency]
    pub btn_hover: Handle<Sample>,
    #[dependency]
    pub btn_press: Handle<Sample>,
    #[dependency]
    pub steps: Vec<Handle<Sample>>,

    // music
    #[dependency]
    pub bg_music: Handle<Sample>,
}

impl AudioSources {
    pub const STEPS: &[&'static str] = &[
        "audio/sfx/step.ogg",
        "audio/sfx/step1.ogg",
        "audio/sfx/step2.ogg",
        "audio/sfx/step3.ogg",
        "audio/sfx/step4.ogg",
    ];
    pub const BTN_HOVER: &'static str = "audio/sfx/btn-hover.ogg";
    pub const BTN_PRESS: &'static str = "audio/sfx/btn-press.ogg";

    pub const BG_MUSIC: &'static str = "audio/music/smnbl-time-for-fun.ogg";
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let steps = Self::STEPS.iter().map(|p| assets.load(*p)).collect();
        Self {
            steps,
            btn_hover: assets.load(Self::STEPS[0]),
            btn_press: assets.load(Self::STEPS[1]),
            bg_music: assets.load(Self::BG_MUSIC),
        }
    }
}
