use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), Player::spawn)
            .add_systems(Update, Player::r#move.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    fn spawn(mut commands: Commands, textures: Res<TextureAssets>) {
        commands.spawn((
            Sprite::from_image(textures.bevy.clone()),
            Transform::from_translation(Vec3::new(0., 0., 1.)),
            Player,
        ));
    }

    fn r#move(
        time: Res<Time>,
        actions: Res<Actions>,
        mut player_query: Query<&mut Transform, With<Player>>,
    ) {
        if actions.player_movement.is_none() {
            return;
        }
        let speed = 150.;
        let movement = Vec3::new(
            actions.player_movement.unwrap().x * speed * time.delta_secs(),
            actions.player_movement.unwrap().y * speed * time.delta_secs(),
            0.,
        );
        for mut player_transform in &mut player_query {
            player_transform.translation += movement;
        }
    }
}
