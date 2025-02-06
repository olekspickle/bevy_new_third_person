use bevy::prelude::*;
use bevy_third_person_camera::*;

use crate::{actions::Actions, Screen};

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn);
}

#[derive(Component)]
pub struct Player;

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //assets: Res<AssetServer>,
) {
    // TODO: load during loading screen
    //let mesh = SceneRoot(assets.load("Player.gltf#Scene0"));
    let mesh = Mesh3d(meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))));
    let color: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
    let pos = Transform::from_translation(Vec3::new(0.0, 0.5, 0.0));
    commands.spawn((color, mesh, pos, ThirdPersonCameraTarget, Player));
}

pub fn movement(
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
