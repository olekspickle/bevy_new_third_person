use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{Screen, despawn};

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), render_description)
        .add_systems(OnExit(Screen::Loading), despawn::<LoadingDescription>);
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            // TODO: Screen::Menu after menu rework
            .continue_to_state(Screen::Playing)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>(),
    );
}

#[derive(Component)]
pub struct LoadingDescription;

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/time-for-fun.ogg")]
    pub bg_play: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "images/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "images/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MeshAssets {
    #[asset(path = "models/Player.gltf")]
    pub player: Handle<Mesh>,
}

fn render_description(mut commands: Commands) {
    commands.spawn((
        Text::new(
            r#"
    See the console for progress output

    This window will close when progress completes..."#,
        ),
        LoadingDescription,
    ));
}
