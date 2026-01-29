use crate::*;
use bevy::asset::Asset;
use bevy_seedling::sample::AudioSample;

mod ron;
mod tracking;

use bevy_shuffle_bag::ShuffleBag;
pub use ron::*;
pub use tracking::*;

pub fn plugin(app: &mut App) {
    // start asset loading
    app.add_plugins(tracking::plugin)
        .add_plugins(RonAssetPlugin::<Config>::default())
        .load_resource_from_path::<Config>("config.ron")
        .add_plugins(RonAssetPlugin::<CreditsPreset>::default())
        .load_resource_from_path::<CreditsPreset>("credits.ron")
        .load_resource::<AudioSources>()
        .load_resource::<Textures>()
        // .load_resource::<Fonts>()
        .load_resource::<Models>();
}

// #[derive(Asset, Clone, Reflect, Resource)]
// #[reflect(Resource)]
// pub struct Fonts {
//     #[dependency]
//     pub custom: Handle<Font>,
// }
//
// impl FromWorld for Fonts {
//     fn from_world(world: &mut World) -> Self {
//         let assets = world.resource::<AssetServer>();
//         Self {
//             custom: assets.load("fonts/custom.ttf"),
//         }
//     }
// }

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct Textures {
    #[dependency]
    pub github: Handle<Image>,
    #[dependency]
    pub pause: Handle<Image>,
    #[dependency]
    pub mute: Handle<Image>,
}

impl FromWorld for Textures {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            github: assets.load("textures/github.png"),
            pause: assets.load("textures/pause.png"),
            mute: assets.load("textures/mute.png"),
        }
    }
}

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct Models {
    #[dependency]
    pub player: Handle<Gltf>,
    #[dependency]
    pub scene: Handle<Gltf>,
}

impl FromWorld for Models {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            player: assets.load("models/player.glb"),
            scene: assets.load("models/scene.glb"),
        }
    }
}

#[derive(Asset, Clone, Reflect, Resource)]
#[reflect(Resource)]
pub struct AudioSources {
    // SFX
    #[dependency]
    pub hover: Handle<AudioSample>,
    #[dependency]
    pub press: Handle<AudioSample>,
    #[dependency]
    pub steps: ShuffleBag<Handle<AudioSample>>,

    // music
    #[dependency]
    pub menu: ShuffleBag<Handle<AudioSample>>,
    #[dependency]
    pub explore: ShuffleBag<Handle<AudioSample>>,
    #[dependency]
    pub combat: ShuffleBag<Handle<AudioSample>>,
}

impl AudioSources {
    pub const BTN_HOVER: &'static str = "audio/sfx/btn-hover.ogg";
    pub const BTN_PRESS: &'static str = "audio/sfx/btn-press.ogg";

    pub const STEPS: &[&'static str] = &[
        "audio/sfx/step.ogg",
        "audio/sfx/step1.ogg",
        "audio/sfx/step2.ogg",
        "audio/sfx/step3.ogg",
        "audio/sfx/step4.ogg",
    ];
    pub const MENU: &[&'static str] = &["audio/music/smnbl-green-embrace.ogg"];
    pub const EXPLORE: &[&'static str] = &["audio/music/smnbl-rush-through-the-field.ogg"];
    pub const COMBAT: &[&'static str] = &["audio/music/smnbl-trouble.ogg"];
}

impl FromWorld for AudioSources {
    fn from_world(world: &mut World) -> Self {
        let mut rng = rand::rng();
        let a = world.resource::<AssetServer>();

        let steps = Self::STEPS.iter().map(|p| a.load(*p)).collect::<Vec<_>>();
        let explore = Self::EXPLORE.iter().map(|p| a.load(*p)).collect::<Vec<_>>();
        let combat = Self::COMBAT.iter().map(|p| a.load(*p)).collect::<Vec<_>>();
        let menu = Self::MENU.iter().map(|p| a.load(*p)).collect::<Vec<_>>();

        Self {
            menu: ShuffleBag::try_new(menu, &mut rng).unwrap(),
            steps: ShuffleBag::try_new(steps, &mut rng).unwrap(),
            combat: ShuffleBag::try_new(combat, &mut rng).unwrap(),
            explore: ShuffleBag::try_new(explore, &mut rng).unwrap(),
            hover: a.load(Self::BTN_HOVER),
            press: a.load(Self::BTN_PRESS),
        }
    }
}
