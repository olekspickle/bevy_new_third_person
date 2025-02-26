use bevy::prelude::*;
use bevy_third_person_camera::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{Action, Screen};

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn)
        .add_systems(Update, movement);
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
    action: Query<&ActionState<Action>>,
    mut camera: Query<&mut Transform, With<ThirdPersonCamera>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let (state, player, camera) = (action.single(), player.single(), camera.single());
    if state.just_pressed(&Action::Right) {}
    //let speed = 150.;

    //let movement = Vec3::new(
    //    actions.player_movement.unwrap().x * speed * time.delta_secs(),
    //    0.,
    //    actions.player_movement.unwrap().y * speed * time.delta_secs(),
    //);
    //for mut player_transform in &mut player_query {
    //    player_transform.translation += movement;
    //}
}

//pub fn set_movement(
//    keyboard_input: Res<ButtonInput<KeyCode>>,
//    touch_input: Res<Touches>,
//    player: Query<&Transform, With<Player>>,
//    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
//) {
//    let right = get_movement(GameControl::Right, &keyboard_input);
//    let left = get_movement(GameControl::Left, &keyboard_input);
//    let up = get_movement(GameControl::Up, &keyboard_input);
//    let down = get_movement(GameControl::Down, &keyboard_input);
//    let mut player_movement = Vec2::new(right - left, down - up);
//
//    if let Some(touch_position) = touch_input.first_pressed_position() {
//        let (camera, camera_transform) = camera.single();
//        if let Ok(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position) {
//            let diff = touch_position - player.single().translation.xy();
//            if diff.length() > FOLLOW_EPSILON {
//                player_movement = diff.normalize();
//            }
//        }
//    }
//
//    if player_movement != Vec2::ZERO {
//        actions.player_movement = Some(player_movement.normalize());
//    } else {
//        actions.player_movement = None;
//    }
//}
