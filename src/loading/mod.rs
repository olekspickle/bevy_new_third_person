use crate::prelude::*;
use bevy::{asset::Asset, prelude::*};

mod ron;
mod tracking;

pub use ron::*;
pub use tracking::*;

pub fn plugin(app: &mut App) {
    // start asset loading
    app.add_plugins(tracking::plugin)
        .add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]))
        .load_resource_from_path::<Config>("config.ron")
        .load_resource::<AudioSources>()
        .load_resource::<Textures>()
        .load_resource::<Models>();
}

// #[reflect(Resource)]
#[derive(Asset, Clone, Reflect, Resource)]
pub struct Textures {
    #[dependency]
    pub github: Handle<Image>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            github: assets.load("textures/github.png"),
        }
    }
}

// #[reflect(Resource)]
#[derive(Asset, Clone, Reflect, Resource)]
pub struct Models {
    #[dependency]
    pub player: Handle<Gltf>,
}

impl FromWorld for Models {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("models/player.glb"),
        }
    }
}

// #[reflect(Resource)]
#[derive(Asset, Clone, Reflect, Resource)]
pub struct AudioSources {
    // SFX
    #[dependency]
    pub btn_hover: Handle<AudioSource>,
    #[dependency]
    pub btn_press: Handle<AudioSource>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,

    // music
    #[dependency]
    pub bg_music: Handle<AudioSource>,
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
