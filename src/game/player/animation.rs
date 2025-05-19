use crate::prelude::*;
use bevy::{prelude::*, scene::SceneInstanceReady};
use bevy_tnua::{
    TnuaAnimatingState, TnuaAnimatingStateDirective,
    builtins::{
        TnuaBuiltinCrouch, TnuaBuiltinCrouchState, TnuaBuiltinDash, TnuaBuiltinJumpState,
        TnuaBuiltinKnockback, TnuaBuiltinKnockbackState,
    },
    prelude::*,
};

const BASE_SPEED_SCALE: f32 = 0.2;

#[derive(Default, Clone)]
pub enum AnimationState {
    #[default]
    StandIdle,
    Run(f32),
    JumpStart,
    JumpLoop,
    JumpLand,
    Fall,
    CrouchWalk(f32),
    CrouchIdle,
    Dash,
    KnockBack,
}

// Bevy's animation handling is a bit manual. We'll use this struct to register the animation clips
// as nodes in the animation graph.
#[derive(Resource)]
pub struct AnimationNodes {
    standing: AnimationNodeIndex,
    running: AnimationNodeIndex,
    jump_start: AnimationNodeIndex,
    jump_loop: AnimationNodeIndex,
    jump_land: AnimationNodeIndex,
    falling: AnimationNodeIndex,
    crouch_walk: AnimationNodeIndex,
    crouch: AnimationNodeIndex,
    dashing: AnimationNodeIndex,
    knockback: AnimationNodeIndex,
}

pub fn prepare_animations(
    _trigger: Trigger<SceneInstanceReady>,
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    mut commands: Commands,
    animation_player: Query<Entity, With<AnimationPlayer>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let Some(gltf) = gltf_assets.get(&models.player) else {
        return;
    };

    let Ok(animation_player_entity) = animation_player.single() else {
        return;
    };

    let mut graph = AnimationGraph::new();
    let root_node = graph.root;
    let nodes = AnimationNodes {
        standing: graph.add_clip(gltf.named_animations["Idle_Loop"].clone(), 1.0, root_node),
        jump_start: graph.add_clip(gltf.named_animations["Jump_Start"].clone(), 1.0, root_node),
        jump_loop: graph.add_clip(gltf.named_animations["Jump_Loop"].clone(), 1.0, root_node),
        jump_land: graph.add_clip(gltf.named_animations["Jump_Land"].clone(), 1.0, root_node),
        falling: graph.add_clip(gltf.named_animations["Jump_Loop"].clone(), 1.0, root_node),
        dashing: graph.add_clip(gltf.named_animations["Roll"].clone(), 1.0, root_node),
        knockback: graph.add_clip(gltf.named_animations["Hit_Chest"].clone(), 1.0, root_node),
        running: graph.add_clip(
            // gltf.named_animations["Jog_Fwd_Loop"].clone(),
            gltf.named_animations["Walk_Loop"].clone(),
            1.0,
            root_node,
        ),
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
    };

    commands.insert_resource(nodes);
    commands
        .entity(animation_player_entity)
        .insert(AnimationGraphHandle(animation_graphs.add(graph)));
}

pub fn animating(
    mut player: Query<&mut Player>,
    mut tnua_controller: Query<(&TnuaController, &mut TnuaAnimatingState<AnimationState>)>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animation_nodes: Option<Res<AnimationNodes>>,
) {
    // An actual game should match the animation player and the controller. Here we cheat for
    // simplicity and use the only controller and only player.
    let Ok((controller, mut animating_state)) = tnua_controller.single_mut() else {
        return;
    };
    let Ok(mut animation_player) = animation_player.single_mut() else {
        return;
    };
    let Ok(mut player) = player.single_mut() else {
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
        Some(TnuaBuiltinKnockback::NAME) => {
            let (_, knockback_state) = controller
                .concrete_action::<TnuaBuiltinKnockback>()
                .expect("action name mismatch: Knockback");
            match knockback_state {
                TnuaBuiltinKnockbackState::Shove => AnimationState::KnockBack,
                TnuaBuiltinKnockbackState::Pushback { .. } => AnimationState::KnockBack,
            }
        }
        Some(TnuaBuiltinCrouch::NAME) => {
            let (_, crouch_state) = controller
                .concrete_action::<TnuaBuiltinCrouch>()
                .expect("action name mismatch: Crouch");

            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                return;
            };
            let basis_speed = basis_state.running_velocity.length();

            // TODO: have transition from/to crouch
            match crouch_state {
                TnuaBuiltinCrouchState::Maintaining => {
                    AnimationState::CrouchWalk(BASE_SPEED_SCALE * basis_speed * player.speed)
                }
                TnuaBuiltinCrouchState::Rising => AnimationState::CrouchIdle,
                TnuaBuiltinCrouchState::Sinking => AnimationState::CrouchIdle,
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
                TnuaBuiltinJumpState::MaintainingJump { .. } => AnimationState::JumpLoop,
                TnuaBuiltinJumpState::StoppedMaintainingJump => AnimationState::JumpLand,
                TnuaBuiltinJumpState::FallSection => AnimationState::Fall,
            }
        }
        Some(TnuaBuiltinDash::NAME) => {
            let (_, _dash_state) = controller
                .concrete_action::<TnuaBuiltinDash>()
                .expect("action name mismatch: Dash");
            // TODO: replace roll with actual dash
            //match dash_state {
            //    _ => AnimationState::Dash,
            //}
            AnimationState::Dash
        }
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
                AnimationState::Fall
            } else {
                let basis_speed = basis_state.running_velocity.length();
                if basis_speed > 0.01 {
                    AnimationState::Run(BASE_SPEED_SCALE * basis_speed * player.speed)
                } else {
                    AnimationState::StandIdle
                }
            }
        }
    };

    // Update player animation state, it could be useful in some systems
    player.animation_state = current_status_for_animating.clone();
    let animating_directive = animating_state.update_by_discriminant(current_status_for_animating);

    match animating_directive {
        TnuaAnimatingStateDirective::Maintain { state } => {
            // `Maintain` means that we did not switch to a different variant, so there is no need
            // to change animations.

            // Specifically for the running animation, even when the state remains the speed can
            // still change. When it does, we simply need to update the speed in the animation
            // player.
            if let AnimationState::Run(speed) = state {
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
                AnimationState::StandIdle => {
                    animation_player
                        .start(animation_nodes.standing)
                        .set_speed(1.0)
                        .repeat();
                }
                AnimationState::Run(speed) => {
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
                        .set_speed(0.01);
                }
                AnimationState::JumpLand => {
                    animation_player
                        .start(animation_nodes.jump_land)
                        .set_speed(0.01);
                }
                AnimationState::JumpLoop => {
                    animation_player
                        .start(animation_nodes.jump_loop)
                        .set_speed(0.5);
                }
                AnimationState::Fall => {
                    animation_player
                        .start(animation_nodes.falling)
                        .set_speed(1.0);
                }
                AnimationState::CrouchWalk(speed) => {
                    animation_player
                        .start(animation_nodes.crouch_walk)
                        .set_speed(*speed);
                }
                AnimationState::CrouchIdle => {
                    animation_player
                        .start(animation_nodes.crouch)
                        .set_speed(1.0);
                }
                AnimationState::Dash => {
                    animation_player
                        .start(animation_nodes.dashing)
                        .set_speed(3.0);
                }
                AnimationState::KnockBack => {
                    animation_player
                        .start(animation_nodes.knockback)
                        .set_speed(1.0);
                }
            }
        }
    }
}
