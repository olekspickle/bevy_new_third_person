use bevy::prelude::*;

use crate::{SceneCamera, Screen, despawn, loading::TextureAssets};

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Menu), (setup_menu, despawn::<SceneCamera>))
        .add_systems(OnExit(Screen::Menu), despawn::<Menu>);
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    //TODO: use bevy_hui for easier UI setup
}
