use crate::prelude::*;
use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_tnua::{
    TnuaAnimatingState, TnuaAnimatingStateDirective,
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinCrouchState, TnuaBuiltinDash, TnuaBuiltinJumpState},
    control_helpers::TnuaSimpleAirActionsCounter,
    prelude::*,
};
use bevy_tnua_avian3d::*;
use leafwing_input_manager::prelude::ActionState;

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        ThirdPersonCameraPlugin,
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));

    app.add_systems(OnEnter(Screen::Gameplay), (prepare_animations, spawn))
        .configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync))
        .add_systems(
            Update,
            (movement, handle_animating)
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(Screen::Gameplay)),
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
    JumpStart,
    JumpLand,
    Falling,
    CrouchWalk,
    Crouch,
}

// Bevy's animation handling is a bit manual. We'll use this struct to register the animation clips
// as nodes in the animation graph.
#[derive(Resource)]
struct AnimationNodes {
    standing: AnimationNodeIndex,
    running: AnimationNodeIndex,
    jump_land: AnimationNodeIndex,
    jump_start: AnimationNodeIndex,
    falling: AnimationNodeIndex,
    crouch_walk: AnimationNodeIndex,
    crouch: AnimationNodeIndex,
}

fn spawn(
    cfg: Res<Config>,
    models: Res<Models>,
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Query<&Transform, With<SceneCamera>>,
) {
    let Some(gltf) = gltf_assets.get(&models.player) else {
        return;
    };
    let camera_transform = camera.single();
    let mut forward = camera_transform.forward().normalize();
    forward.y = 0.0;
    let player_rot = Quat::from_rotation_y(PI);

    let mesh = SceneRoot(gltf.scenes[0].clone());
    let pos = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_rotation(player_rot);

    let collider = Collider::capsule(cfg.player.hitbox.radius, cfg.player.hitbox.height);
    let collider_mesh = Mesh::from(Capsule3d::new(
        cfg.player.hitbox.radius,
        cfg.player.hitbox.height,
    ));
    let debug_collider_mesh = Mesh3d(meshes.add(collider_mesh.clone()));
    let debug_collider_color: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.2)));

    commands.spawn((
        mesh,
        pos,
        Player::default(),
        ThirdPersonCameraTarget,
        // tnua stuff
        TnuaController::default(),
        // Tnua can fix the rotation, but the character will still get rotated before it can do so.
        // By locking the rotation we can prevent this.
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
        TnuaAnimatingState::<AnimationState>::default(),
        TnuaSimpleAirActionsCounter::default(),
        // physics
        // A sensor shape is not strictly necessary, but without it we'll get weird results.
        TnuaAvian3dSensorShape(collider.clone()),
        RigidBody::Dynamic,
        collider,
        debug_collider_mesh,
        debug_collider_color,
    ));
}

pub fn movement(
    cfg: Res<Config>,
    time: Res<Time<Virtual>>,
    //touch_input: Res<Touches>,
    action: Query<&ActionState<Action>>,
    mut tnua: Query<&mut TnuaController>,
    mut air_counter: Query<&mut TnuaSimpleAirActionsCounter>,
    camera: Query<&Transform, With<SceneCamera>>,
) {
    let Ok(mut controller) = tnua.get_single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;
    let speed = cfg.player.movement.speed * time.delta_secs();

    let (state, camera_transform) = (action.single(), camera.single());
    let forward = camera_transform.forward().normalize();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z);

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
        direction += forward_flat;
    }

    if state.pressed(&Action::Backward) {
        let back = camera_transform.back().normalize();
        let back_flat = Vec3::new(back.x, 0.0, back.z);
        direction += back_flat;
    }

    // NOTE: subject to change. UAL model is imported rotated 180 so we rotate it back
    let player_rot = Quat::from_rotation_y(PI) * direction;
    // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
    // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
    // just fall.
    controller.basis(TnuaBuiltinWalk {
        // The `desired_velocity` determines how the character will move.
        desired_velocity: direction.normalize_or_zero() * speed,
        desired_forward: Dir3::new(player_rot).ok(),
        // The `float_height` must be greater (even if by little) from the distance between the
        // character's center and the lowest point of its collider.
        float_height: 0.0001,
        // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
        // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
        ..Default::default()
    });

    // Feed the jump action every frame as long as the player holds the jump button. If the player
    // stops holding the jump button, simply stop feeding the action.
    let mut air_counter = air_counter.single_mut();
    air_counter.update(controller.as_mut());

    if state.just_pressed(&Action::Jump) {
        //let jump = jump.single();
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 4.0,
            allow_in_air: true,
            ..Default::default()
        });
    }

    if state.just_pressed(&Action::Dash) {
        controller.action(TnuaBuiltinDash {
            // Dashing is also an action, but because it has directions we need to provide said
            // directions. `displacement` is a vector that determines where the jump will bring
            // us. Note that even after reaching the displacement, the character may still have
            // some leftover velocity (configurable with the other parameters of the action)
            //
            // The displacement is "frozen" when the action starts - user code does not have to
            // worry about storing the original direction
            displacement: direction.normalize() * cfg.player.movement.dash_distance,
            // When set, the `desired_forward` of the dash action "overrides" the
            // `desired_forward` of the walk basis. Like the displacement, it gets "frozen" -
            // allowing to easily maintain a forward direction during the dash.
            desired_forward: Dir3::new(player_rot).ok(),
            allow_in_air: air_counter.air_count_for(TnuaBuiltinDash::NAME)
                <= cfg.player.movement.actions_in_air.into(),
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
        standing: graph.add_clip(gltf.named_animations["Idle_Loop"].clone(), 1.0, root_node),
        running: graph.add_clip(
            gltf.named_animations["Jog_Fwd_Loop"].clone(),
            1.0,
            root_node,
        ),
        jump_start: graph.add_clip(gltf.named_animations["Jump_Start"].clone(), 1.0, root_node),
        jump_land: graph.add_clip(gltf.named_animations["Jump_Land"].clone(), 1.0, root_node),
        falling: graph.add_clip(gltf.named_animations["Jump_Loop"].clone(), 1.0, root_node),
        crouch_walk: graph.add_clip(
            gltf.named_animations["Crouch_Fwd_Loop"].clone(),
            1.0,
            root_node,
        ),
        crouch: graph.add_clip(
            gltf.named_animations["Crouch_Idle_Loop"].clone(),
            1.0,
            root_node,
        ),
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
        Some(TnuaBuiltinCrouch::NAME) => {
            let (_, crouch_state) = controller
                .concrete_action::<TnuaBuiltinCrouch>()
                .expect("action name mismatch");
            // TODO: have transition from/to crouch
            match crouch_state {
                TnuaBuiltinCrouchState::Maintaining => AnimationState::Crouch,
                TnuaBuiltinCrouchState::Rising => AnimationState::Crouch,
                TnuaBuiltinCrouchState::Sinking => AnimationState::Crouch,
            }
        }
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
                TnuaBuiltinJumpState::StartingJump { .. } => AnimationState::JumpStart,
                TnuaBuiltinJumpState::SlowDownTooFastSlopeJump { .. } => AnimationState::JumpStart,
                TnuaBuiltinJumpState::MaintainingJump => AnimationState::Falling,
                TnuaBuiltinJumpState::StoppedMaintainingJump => AnimationState::JumpLand,
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
                AnimationState::JumpStart => {
                    animation_player
                        .start(animation_nodes.jump_start)
                        .set_speed(1.0);
                }
                AnimationState::JumpLand => {
                    animation_player
                        .start(animation_nodes.jump_land)
                        .set_speed(1.0);
                }
                AnimationState::Falling => {
                    animation_player
                        .start(animation_nodes.falling)
                        .set_speed(1.0);
                }
                AnimationState::CrouchWalk => {
                    animation_player
                        .start(animation_nodes.crouch_walk)
                        .set_speed(1.0);
                }
                AnimationState::Crouch => {
                    animation_player
                        .start(animation_nodes.crouch)
                        .set_speed(1.0);
                }
            }
        }
    }
}
