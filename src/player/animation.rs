//! Main animation system
//!
//! My idea is based on tnua approach - detect if we are to alter the animation state
//! and then change the animation state accordingly
//!
//! Create a new animation graph for each player, start all animations with 0 weight and add weight
//! exponentially between frames based on input
use super::*;
use bevy::scene::SceneInstanceReady;

/// Animation control knobs
mod knobs {
    pub const TRANSITION_DURATION: f32 = 0.15;
    pub const NORMAL: f32 = 1.0;
    pub const GENERAL_SPEED: f32 = 0.1;
    pub const CROUCH_SPEED: f32 = 2.2;
    pub const JUMP_SPEED: f32 = 0.01;
    pub const CLIMB_SPEED: f32 = 0.3;
    /// sprint animation to sprint factor ratio
    pub const SPRINT_SPEED: f32 = 3.0;
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, animating.run_if(in_state(Screen::Gameplay)));
}

pub fn prepare_animations(
    spawned: On<SceneInstanceReady>,
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    children_q: Query<&Children>,
    animation_player_q: Query<Entity, With<AnimationPlayer>>,
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    info!("prepare_animations observer fired on: {}", spawned.entity);

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
    info!("found animation player: {}", e);
    let Ok(mut player) = player.single_mut() else {
        return;
    };

    let mut graph = AnimationGraph::new();

    // Create flat animation graph with one blend node
    for (name, clip) in gltf.named_animations.iter() {
        let node_index = graph.add_clip(clip.clone(), 0.0, graph.root);
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
    animation_players_q: Query<Entity, With<AnimationPlayer>>,
    mut player_q: Query<&mut Player>,
    // mut player_q: Query<(&mut Player, &mut AnimationTransitions)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    // for (mut player, mut animation_transitions) in player_q {
    for mut player in player_q {
        let Some(e) = player
            .id
            .get_animation_player_e(children_q, animation_players_q)
        else {
            continue;
        };
        let Ok(mut animation_player) = animation_players.get_mut(e) else {
            continue;
        };

        animation_player.zero_all_animations();

        if player.animation.alter() {
            match player.animation.next() {
                AnimationState::StandIdle => {
                    if let Some(index) = player.animation.map.get("Idle_Loop") {
                        // animation_transitions
                        //     .start(
                        //         &mut animation_player,
                        //         *index,
                        //         Duration::from_secs_f32(knobs::TRANSITION_DURATION),
                        //     )
                        //     .set_weight(1.0)
                        //     .set_speed(knobs::NORMAL)
                        //     .repeat();
                        animation_player
                            .start(*index)
                            .set_weight(1.0)
                            .set_speed(knobs::NORMAL)
                            .repeat();
                    }
                }
                AnimationState::Run(speed) => {
                    if let Some(index) = player.animation.map.get("Jog_Fwd_Loop") {
                        animation_player
                            .start(*index)
                            .set_weight(1.0)
                            .set_speed(speed)
                            .repeat();
                    }
                }
                AnimationState::Sprint(speed) => {
                    if let Some(index) = player.animation.map.get("Sprint_Loop") {
                        animation_player
                            .start(*index)
                            .set_weight(1.0)
                            .set_speed(speed * knobs::SPRINT_SPEED)
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
                            .set_speed(knobs::NORMAL)
                            .repeat();
                    }
                }
                AnimationState::Fall => {
                    if let Some(index) = player.animation.map.get("Jump_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(knobs::NORMAL)
                            .repeat();
                    }
                }
                AnimationState::Crouch(speed) => {
                    if let Some(index) = player.animation.map.get("Crouch_Fwd_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(speed * knobs::CROUCH_SPEED)
                            .repeat();
                    }
                }
                AnimationState::CrouchIdle => {
                    if let Some(index) = player.animation.map.get("Crouch_Idle_Loop") {
                        animation_player
                            .start(*index)
                            .set_speed(knobs::NORMAL)
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
                        animation_player.start(*index).set_speed(knobs::NORMAL);
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
}
