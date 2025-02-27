use bevy::prelude::*;
use bevy_third_person_camera::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{Action, Screen, camera::SceneCamera};

pub const FOLLOW_EPSILON: f32 = 5.;

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `GameState::Playing`
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn);
    //.add_systems(Update, movement.run_if(in_state(Screen::Playing)));
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
    time: Res<Time<Virtual>>,
    //touch_input: Res<Touches>,
    action: Query<&ActionState<Action>>,
    camera: Query<&mut Transform, With<SceneCamera>>,
    mut player: Query<&mut Transform, (With<Player>, Without<SceneCamera>)>,
) {
    let (state, mut player) = (action.single(), player.single_mut());
    let camera_transform = camera.single();
    let speed = 3.0 * time.delta_secs();
    let mut direction = Vec3::ZERO;

    if state.just_pressed(&Action::Right) {
        let right = camera_transform.right().normalize();
        let right_flat = Vec3::new(right.x, 0.0, right.z).normalize();
        direction += speed * right_flat;
    }

    if state.just_pressed(&Action::Left) {
        let left = camera_transform.left().normalize();
        let left_flat = Vec3::new(left.x, 0.0, left.z).normalize();
        direction += speed * left_flat;
    }

    if state.just_pressed(&Action::Forward) {
        let forward = camera_transform.forward().normalize();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize();
        direction += speed * forward_flat;
    }

    if state.just_pressed(&Action::Backward) {
        let back = camera_transform.back().normalize();
        let back_flat = Vec3::new(back.x, 0.0, back.z).normalize();
        direction += speed * back_flat;
    }

    // TODO: jump

    //if let Some(touch_position) = touch_input.first_pressed_position() {
    //    if let Ok(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position) {
    //        let diff = touch_position - player.translation.xy();
    //        if diff.length() > FOLLOW_EPSILON {
    //            player_movement = diff.normalize();
    //        }
    //    }
    //}

    if direction.length_squared() > 0.0 {
        direction = direction.normalize(); // Normalize to avoid diagonal speed boost
        player.translation += direction * speed;
    }
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
