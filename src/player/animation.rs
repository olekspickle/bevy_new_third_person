//! Main animation system
//!
//! My idea is based on tnua approach - detect if we are to alter the animation state
//! and then change the animation state accordingly
//!
//! Create a new animation graph for each player, start all animations with 0 weight and add weight
//! exponentially between frames based on input
use super::*;
use bevy::scene::SceneInstanceReady;
use bevy_ahoy::CharacterControllerOutput;
use std::time::Duration;

/// Animation control knobs
mod knobs {
    // pub const TRANSITION_DURATION: f32 = 0.15;
    pub const DAMPENING: f32 = 0.01;
    pub const JUMP_SPEED: f32 = 0.01;
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, animating.run_if(in_state(Screen::Gameplay)));
}
pub fn prepare_animations(
    spawned: On<SceneInstanceReady>,
    // TODO: try this
    // spawned: On<Add, AnimationPlayer>,
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    children_q: Query<&Children>,
    animation_player_q: Query<Entity, With<AnimationPlayer>>,
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("prepare_animations fired on: {}", spawned.entity);

    let Some(gltf) = gltf_assets.get(&models.player) else {
        return;
    };
    let Some(e) = spawned
        .entity
        .get_animation_player_e(children_q, animation_player_q)
    else {
        return;
    };
    let Ok(mut animation_player) = animation_players.get_mut(e) else {
        return;
    };
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    let mut graph = AnimationGraph::new();

    // Create flat animation graph with one blend node
    for (name, clip) in gltf.named_animations.iter() {
        let node_index = graph.add_clip(clip.clone(), 1.0, graph.root);
        player.animation.map.insert(name.to_string(), node_index);
    }

    // TODO: check if it still works on the second gamepad
    // Add animation graph to animation player
    commands
        .entity(e)
        .insert(AnimationGraphHandle(animation_graphs.add(graph)));

    for i in player.animation.map.values() {
        animation_player.start(*i).repeat();
    }
}

pub fn animating(
    cfg: Res<Config>,
    children_q: Query<&Children>,
    player_q: Query<(
        &mut Player,
        &mut StepTimer,
        &CharacterController,
        &CharacterControllerOutput,
    )>,
    movement: Single<&Action<Movement>>,
    camera: Query<&Transform, With<SceneCamera>>,
    animation_players_q: Query<Entity, With<AnimationPlayer>>,
    mut animation_players: Query<&mut AnimationPlayer>,
) -> Result {
    let movement = *movement.into_inner();

    for (mut player, mut step_timer, ahoy, ahoy_out) in player_q {
        let Some(e) = player
            .id
            .get_animation_player_e(children_q, animation_players_q)
        else {
            continue;
        };
        let Ok(mut animation_player) = animation_players.get_mut(e) else {
            continue;
        };

        let cam_transform = camera.single()?;
        let input_dir = cam_transform.movement_direction(*movement);

        let moving = input_dir.length_squared() > cfg.player.movement.idle_to_run_threshold;
        match player.animation.state.0 {
            AnimationState::StandIdle if moving => {
                info!("moving from idle");

                // update step timer dynamically based on actual speed
                // Note: this is specific to the animation provided
                // normal step: 0.475
                // sprint step (x1.5): 0.354
                // step on sprint timer: 0.317
                let ratio = cfg.player.movement.speed() / ahoy.speed;
                let adjusted_step_time_f32 = cfg.timers.step * ratio;
                let adjusted_step_time = Duration::from_secs_f32(adjusted_step_time_f32);
                // info!("step timer:{adjusted_step_time_f32}s");
                step_timer.set_duration(adjusted_step_time);
                player.animation.run(ahoy.speed);
            }
            AnimationState::Run(_) if !moving => {
                info!("idling after run");
                player.animation.idle()
            }
            // touching entities after jumping
            anim if !ahoy_out.touching_entities.is_empty() && player.animation.is_jumping() => {
                info!("landing after jump or fall");
                match anim {
                    AnimationState::JumpLoop => player.animation.jump_land(),
                    _ => player.animation.jump_start(),
                }
            }
            anim if ahoy_out.touching_entities.is_empty() && player.animation.is_jumping() => {
                if matches!(anim, AnimationState::JumpStart)
                    && !matches!(anim, AnimationState::JumpLoop)
                {
                    player.animation.jump_loop()
                }
            }
            anim if ahoy_out.touching_entities.is_empty() && !player.animation.is_jumping() => {
                player.animation.jump_start()
            }
            _ => (),
        }
        // info!("next anim: {:?}", player.animation.state.1);

        animation_player.zero_all_animations();
        if player.animation.alter() {
            match player.animation.next() {
                AnimationState::StandIdle => {
                    if let Some(index) = player.animation.map.get("Idle_Loop") {
                        animation_player.start(*index).set_weight(1.0).repeat();
                    }
                }
                AnimationState::Run(speed) => {
                    if let Some(index) = player.animation.map.get("Jog_Fwd_Loop") {
                        info!("Running: {speed}, res speed: {}", speed * knobs::DAMPENING);
                        animation_player
                            .start(*index)
                            .set_weight(1.0)
                            .set_speed(speed * knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::Sprint(speed) => {
                    if let Some(index) = player.animation.map.get("Sprint_Loop") {
                        animation_player
                            .start(*index)
                            .set_weight(1.0)
                            .set_speed(speed * knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::JumpStart => {
                    if let Some(index) = player.animation.map.get("Jump_Start") {
                        animation_player.start(*index).set_speed(knobs::JUMP_SPEED);
                    }
                }
                AnimationState::JumpLand => {
                    if let Some(index) = player.animation.map.get("Jump_Land") {
                        animation_player.start(*index).set_speed(knobs::JUMP_SPEED);
                    }
                }
                AnimationState::JumpLoop => {
                    if let Some(index) = player.animation.map.get("Jump_Loop") {
                        animation_player.start(*index).set_speed(0.5).repeat();
                    }
                }
                AnimationState::WallJump => {
                    if let Some(index) = player.animation.map.get("Jump_Start") {
                        animation_player.start(*index).set_speed(2.0);
                    }
                }
                AnimationState::WallSlide => {
                    if let Some(index) = player.animation.map.get("Jump_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::Fall => {
                    if let Some(index) = player.animation.map.get("Jump_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::Crouch(speed) => {
                    if let Some(index) = player.animation.map.get("Crouch_Fwd_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(speed * knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::CrouchIdle => {
                    if let Some(index) = player.animation.map.get("Crouch_Idle_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(knobs::DAMPENING)
                            .repeat();
                    }
                }
                AnimationState::Dash => {
                    if let Some(index) = player.animation.map.get("Roll") {
                        animation_player.start(*index).set_speed(3.0);
                    }
                }
                AnimationState::Knockback => {
                    if let Some(index) = player.animation.map.get("Hit_Chest") {
                        animation_player.start(*index).set_speed(knobs::DAMPENING);
                    }
                }
                AnimationState::Climb(speed) => {
                    if let Some(index) = player.animation.map.get("Jump_Loop") {
                        animation_player.start(*index).set_speed(speed).repeat();
                    }
                }
            }
            player.animation.state.0 = player.animation.state.1;
        }
    }

    Ok(())
}
