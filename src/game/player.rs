use crate::prelude::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_tnua::{
    TnuaAnimatingState, TnuaAnimatingStateDirective, builtins::TnuaBuiltinJumpState, prelude::*,
};
use bevy_tnua_avian3d::*;
use leafwing_input_manager::prelude::ActionState;

const ZOOM: (f32, f32) = (1.5, 30.);

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        ThirdPersonCameraPlugin,
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));
    app.add_systems(OnEnter(Screen::Playing), spawn)
        .add_systems(OnExit(Screen::Playing), despawn::<ThirdPersonCamera>)
        .add_systems(
            Update,
            (
                movement.in_set(TnuaUserControlsSystemSet),
                //prepare_animations,
                //handle_animating,
            )
                .run_if(in_state(Screen::Playing)),
        );
}

#[derive(Component, Default)]
pub struct Player {
    animation_state: AnimationState,
}

#[derive(Default)]
pub enum AnimationState {
    #[default]
    Standing,
    Running(f32),
    Jumping,
    Falling,
}

// Bevy's animation handling is a bit manual. We'll use this struct to register the animation clips
// as nodes in the animation graph.
#[derive(Resource)]
struct AnimationNodes {
    standing: AnimationNodeIndex,
    running: AnimationNodeIndex,
    jumping: AnimationNodeIndex,
    falling: AnimationNodeIndex,
}

fn spawn(
    meshes: Res<Models>,
    //mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    //assets: Res<AssetServer>,
    gltf_assets: Res<Assets<Gltf>>,
    mut camera: Query<Entity, With<SceneCamera>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera = camera.single_mut();
    commands.entity(camera).insert(ThirdPersonCamera {
        aim_speed: 3.0,                  // default
        aim_zoom: 0.7,                   // default
        zoom_enabled: true,              // default
        zoom: Zoom::new(ZOOM.0, ZOOM.1), // default
        aim_enabled: true,
        offset_enabled: true,
        offset_toggle_enabled: true,
        gamepad_settings: CustomGamepadSettings::default(),
        ..default()
    });

    let Some(gltf) = gltf_assets.get(&meshes.player) else {
        return;
    };
    let mesh = SceneRoot(gltf.scenes[0].clone());
    //let mesh = Mesh3d(meshes.add(Cylinder::new(10., 10.)));
    let color: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
    let pos = Transform::from_translation(Vec3::new(0.0, 0.5, 0.0));
    commands.spawn((
        color,
        mesh,
        pos,
        Player::default(),
        ThirdPersonCameraTarget,
        // tnua stuff
        TnuaController::default(),
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        TnuaAnimatingState::<AnimationState>::default(),
        // physics
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
    ));
}

pub fn movement(
    time: Res<Time<Virtual>>,
    //touch_input: Res<Touches>,
    action: Query<&ActionState<Action>>,
    mut tnua: Query<&mut TnuaController>,
    camera: Query<&mut Transform, With<SceneCamera>>,
    mut player: Query<&mut Transform, (With<Player>, Without<SceneCamera>)>,
) {
    let Ok(mut controller) = tnua.get_single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    let speed = 50.0 * time.delta_secs();

    let (state, camera_transform, mut player) =
        (action.single(), camera.single(), player.single_mut());
    let forward = camera_transform.forward().normalize();

    // Rotate player
    //let mut forward_dir = forward;
    //forward_dir.y = 0.0;
    //let player_rot = Quat::from_rotation_arc(Vec3::X, forward_dir);
    //player.rotation = player_rot;

    if state.pressed(&Action::Right) {
        let right = camera_transform.right().normalize();
        let right_flat = Vec3::new(right.x, 0.0, right.z);
        direction += right_flat;
    }

    if state.pressed(&Action::Left) {
        let left = camera_transform.left().normalize();
        let left_flat = Vec3::new(left.x, 0.0, left.z);
        direction += left_flat;
    }

    if state.pressed(&Action::Forward) {
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z);
        direction += forward_flat;
    }

    if state.pressed(&Action::Backward) {
        let back = camera_transform.back().normalize();
        let back_flat = Vec3::new(back.x, 0.0, back.z);
        direction += back_flat;
        //player.rotate_y(2. * rot.angle * time.delta_secs());
    }

    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * speed,
        desired_forward: Dir3::new(direction).ok(),
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 2.0,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // TODO: jump
    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    if state.pressed(&Action::Jump) {
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            // `TnuaBuiltinJump` also has customization fields with sensible defaults.
            ..Default::default()
        });
    }

    //if let Some(touch_position) = touch_input.first_pressed_position() {
    //    if let Ok(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position) {
    //        let diff = touch_position - player.translation.xy();
    //        if diff.length() > FOLLOW_EPSILON {
    //            player_movement = diff.normalize();
    //        }
    //    }
    //}
}

fn prepare_animations(
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    mut commands: Commands,
    animation_player_query: Query<Entity, With<AnimationPlayer>>,
    mut animation_graphs_assets: ResMut<Assets<AnimationGraph>>,
) {
    let Some(gltf) = gltf_assets.get(&models.player) else {
        return;
    };
    let Ok(animation_player_entity) = animation_player_query.get_single() else {
        return;
    };

    let mut graph = AnimationGraph::new();
    let root_node = graph.root;

    commands.insert_resource(AnimationNodes {
        standing: graph.add_clip(gltf.named_animations["Standing"].clone(), 1.0, root_node),
        running: graph.add_clip(gltf.named_animations["Running"].clone(), 1.0, root_node),
        jumping: graph.add_clip(gltf.named_animations["Jumping"].clone(), 1.0, root_node),
        falling: graph.add_clip(gltf.named_animations["Falling"].clone(), 1.0, root_node),
    });

    commands
        .entity(animation_player_entity)
        .insert(AnimationGraphHandle(animation_graphs_assets.add(graph)));
}

fn handle_animating(
    mut player_query: Query<(&TnuaController, &mut TnuaAnimatingState<AnimationState>)>,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    animation_nodes: Option<Res<AnimationNodes>>,
) {
    // An actual game should match the animation player and the controller. Here we cheat for
    // simplicity and use the only controller and only player.
    let Ok((controller, mut animating_state)) = player_query.get_single_mut() else {
        return;
    };
    let Ok(mut animation_player) = animation_player_query.get_single_mut() else {
        return;
    };
    let Some(animation_nodes) = animation_nodes else {
        return;
    };

    // Here we use the data from TnuaController to determine what the character is currently doing,
    // so that we can later use that information to decide which animation to play.

    // First we look at the `action_name` to determine which action (if at all) the character is
    // currently performing:
    let current_status_for_animating = match controller.action_name() {
        // Unless you provide the action names yourself, prefer matching against the `NAME` const
        // of the `TnuaAction` trait. Once `type_name` is stabilized as `const` Tnua will use it to
        // generate these names automatically, which may result in a change to the name.
        Some(TnuaBuiltinJump::NAME) => {
            // In case of jump, we want to cast it so that we can get the concrete jump state.
            let (_, jump_state) = controller
                .concrete_action::<TnuaBuiltinJump>()
                .expect("action name mismatch");
            // Depending on the state of the jump, we need to decide if we want to play the jump
            // animation or the fall animation.
            match jump_state {
                TnuaBuiltinJumpState::NoJump => return,
                TnuaBuiltinJumpState::StartingJump { .. } => AnimationState::Jumping,
                TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. } => AnimationState::Jumping,
                TnuaBuiltinJumpState::MaintainingJump => AnimationState::Jumping,
                TnuaBuiltinJumpState::StoppedMaintainingJump => AnimationState::Jumping,
                TnuaBuiltinJumpState::FallSection => AnimationState::Falling,
            }
        }
        // Tnua should only have the `action_name` of the actions you feed to it. If it has
        // anything else - consider it a bug.
        Some(other) => panic!("Unknown action {other}"),
        // No action name means that no action is currently being performed - which means the
        // animation should be decided by the basis.
        None => {
            // If there is no action going on, we'll base the animation on the state of the
            // basis.
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                // Since we only use the walk basis in this example, if we can't get get this
                // basis' state it probably means the system ran before any basis was set, so we
                // just stkip this frame.
                return;
            };
            if basis_state.standing_on_entity().is_none() {
                // The walk basis keeps track of what the character is standing on. If it doesn't
                // stand on anything, `standing_on_entity` will be empty - which means the
                // character has walked off a cliff and needs to fall.
                AnimationState::Falling
            } else {
                let speed = basis_state.running_velocity.length();
                if 0.01 < speed {
                    AnimationState::Running(0.1 * speed)
                } else {
                    AnimationState::Standing
                }
            }
        }
    };

    let animating_directive = animating_state.update_by_discriminant(current_status_for_animating);

    match animating_directive {
        TnuaAnimatingStateDirective::Maintain { state } => {
            // `Maintain` means that we did not switch to a different variant, so there is no need
            // to change animations.

            // Specifically for the running animation, even when the state remains the speed can
            // still change. When it does, we simply need to update the speed in the animation
            // player.
            if let AnimationState::Running(speed) = state {
                if let Some(animation) = animation_player.animation_mut(animation_nodes.running) {
                    animation.set_speed(*speed);
                }
            }
        }
        TnuaAnimatingStateDirective::Alter {
            old_state: _,
            state,
        } => {
            // `Alter` means that we have switched to a different variant and need to play a
            // different animation.

            // First - stop the currently running animation. We don't check which one is running
            // here because we just assume it belongs to the old state, but more sophisticated code
            // can try to phase from the old animation to the new one.
            animation_player.stop_all();

            // Depending on the new state, we choose the animation to run and its parameters (here
            // they are the speed and whether or not to repeat)
            match state {
                AnimationState::Standing => {
                    animation_player
                        .start(animation_nodes.standing)
                        .set_speed(1.0)
                        .repeat();
                }
                AnimationState::Running(speed) => {
                    animation_player
                        .start(animation_nodes.running)
                        // The running animation, in particular, has a speed that depends on how
                        // fast the character is running. Note that if the speed changes while the
                        // character is still running we won't get `Alter` again - so it's
                        // important to also update the speed in `Maintain { State: Running }`.
                        .set_speed(*speed)
                        .repeat();
                }
                AnimationState::Jumping => {
                    animation_player
                        .start(animation_nodes.jumping)
                        .set_speed(2.0);
                }
                AnimationState::Falling => {
                    animation_player
                        .start(animation_nodes.falling)
                        .set_speed(1.0);
                }
            }
        }
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
