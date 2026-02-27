//! Main animation system
//!
//! My idea is based on tnua approach - detect if we are to alter the animation state
//! and then change the animation state accordingly
//!
//! Create a new animation graph for each player, start all animations with 0 weight and add weight
//! exponentially between frames based on input
use super::*;
use bevy::scene::SceneInstanceReady;
use std::time::Duration;

/// Animation control knobs
mod knobs {
    /// General damping factor to slow down animations using speed
    pub const DAMPING: f32 = 0.05;
    pub const TRANSITION: f32 = 0.2;
    /// Use a slightly faster transition for jumps to make them feel "snappy"
    pub const JUMP_TRANSITION: f32 = 0.1;
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (calcucate_animations, animate)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
}

/// Build animation graph when scene loads
pub fn prepare_animations(
    on: On<SceneInstanceReady>,
    models: Res<Models>,
    gltfs: Res<Assets<Gltf>>,
    children_q: Query<&Children>,
    animation_player_q: Query<Entity, With<AnimationPlayer>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut players: Query<&mut Player>,
    mut commands: Commands,
) {
    let Some(animation_player_e) = on.entity.get_recursive(children_q, animation_player_q) else {
        return;
    };
    let Ok(mut animation_player) = animation_players.get_mut(animation_player_e) else {
        return;
    };

    let Some(gltf) = gltfs.get(&models.player) else {
        return;
    };

    // we list acnimations here in the same order they are listed in AnimationState enum
    let clips = vec![
        gltf.named_animations["Idle_Loop"].clone(),
        gltf.named_animations["Jog_Fwd_Loop"].clone(),
        gltf.named_animations["Sprint_Loop"].clone(),
        gltf.named_animations["Jump_Start"].clone(),
        gltf.named_animations["Jump_Loop"].clone(),
        gltf.named_animations["Jump_Land"].clone(),
        gltf.named_animations["Crouch_Fwd_Loop"].clone(),
        gltf.named_animations["Crouch_Idle_Loop"].clone(),
        gltf.named_animations["Roll"].clone(),
    ];

    let (graph, nodes) = AnimationGraph::from_clips(clips);
    let graph_handle = graphs.add(graph);

    commands
        .entity(animation_player_e)
        .insert(AnimationGraphHandle(graph_handle))
        .insert(AnimationTransitions::default());

    let idle_node = nodes[0];
    animation_player.play(idle_node).repeat();

    if let Ok(mut player) = players.single_mut() {
        debug!("adding animations to player: {}", player.id);
        player.animation = Animations {
            // state: (AnimationState::StandIdle, AnimationState::StandIdle),
            current: AnimationState::StandIdle,
            requested: None,
            nodes,
            animation_player_e,
        };
    }
}

pub fn calcucate_animations(
    time: Res<Time>,
    cfg: Res<Config>,
    mut players: Query<(
        &CharacterController,
        &CharacterControllerState,
        &Transform,
        &mut PreviousPosition,
        &mut Player,
    )>,
) {
    let idle_to_run_ani = cfg.player.movement.idle_to_run_threshold * 1000.0;

    for (ahoy, ahoy_state, pos, mut prev_pos, mut player) in players.iter_mut() {
        let animation = &mut player.animation;

        let displacement = pos.translation - prev_pos.0;
        let velocity = displacement / time.delta_secs();
        let h_speed = Vec3::new(velocity.x, 0.0, velocity.z).length().abs();
        let v_speed = velocity.y;
        prev_pos.0 = pos.translation;
        // debug!("v_speed: {v_speed}, h_speed: {h_speed}");

        let grounded = ahoy_state.grounded.is_some();

        // MANTLE
        if ahoy_state.mantle.is_some() {
            animation.request(AnimationState::Dash); // or Mantle
            continue;
        }

        // in the air animation
        if !grounded {
            if !animation.current.is_jumping() {
                debug!(
                    "jumping,v_speed: {v_speed}, h_speed: {h_speed}, elapsed: {:?}",
                    ahoy_state.last_ground.elapsed().as_secs_f32()
                );
                if v_speed > 0.1 {
                    animation.request(AnimationState::Jump);
                } else {
                    animation.request(AnimationState::JumpLoop);
                }
            }
            continue;
        }

        // at this point we are grounded
        if grounded && animation.current.is_jumping() {
            if h_speed > idle_to_run_ani {
                // Landed while running? Skip the thud, go to Run
                animation.request(AnimationState::Run(h_speed));
            } else {
                animation.request(AnimationState::Land);
            }
            continue;
        }

        // CROUCH
        if ahoy_state.crouching {
            if h_speed > idle_to_run_ani {
                animation.request(AnimationState::Crouch(h_speed));
            } else {
                animation.request(AnimationState::CrouchIdle);
            }
            continue;
        }

        // SPRINT
        let is_sprinting = ahoy.speed > cfg.player.movement.speed();
        if is_sprinting {
            animation.request(AnimationState::Sprint(h_speed));
            continue;
        }

        // and finally RUN\IDLE
        if h_speed > idle_to_run_ani {
            animation.request(AnimationState::Run(h_speed));
        } else {
            animation.request(AnimationState::StandIdle);
        }
    }
}

pub fn animate(
    mut players: Query<&mut Player>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut transitions_query: Query<&mut AnimationTransitions>,
) {
    for mut player in players.iter_mut() {
        let ani = &mut player.animation;
        let Ok(mut animation_player) = animation_players.get_mut(ani.animation_player_e) else {
            continue;
        };

        let Ok(mut transitions) = transitions_query.get_mut(ani.animation_player_e) else {
            continue;
        };

        if let Some(next) = ani.requested.take() {
            let node = ani.nodes[next.clip_index()];

            let duration = if next.is_jumping() {
                knobs::JUMP_TRANSITION
            } else {
                knobs::TRANSITION
            };

            transitions
                .play(
                    &mut animation_player,
                    node,
                    Duration::from_secs_f32(duration),
                )
                .repeat();

            let current_node = ani.nodes[ani.current.clip_index()];
            if let Some(active) = animation_player.animation_mut(current_node) {
                match ani.current {
                    AnimationState::Run(s)
                    | AnimationState::Sprint(s)
                    | AnimationState::Crouch(s) => active.set_speed(s * knobs::DAMPING),
                    _ => active.set_speed(1.0),
                };
            }

            debug!("current: {:?}, next: {next:?}", ani.current);
            ani.current = next;
        }

        if ani.current.is_locked() {
            let node = ani.nodes[ani.current.clip_index()];

            if let Some(active) = animation_player.animation(node)
                && active.elapsed() >= active.seek_time() * 0.85
            {
                let chained = match ani.current {
                    AnimationState::Jump => Some(AnimationState::JumpLoop),
                    AnimationState::Land => Some(AnimationState::StandIdle),
                    _ => None,
                };

                if let Some(next) = chained {
                    debug!("chained: {next:?}");
                    ani.requested = Some(next);
                }
            }
        }
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Animations {
    /// (current, next)
    // pub state: (AnimationState, AnimationState),
    pub current: AnimationState,
    pub requested: Option<AnimationState>,
    /// AnimationState → Graph node
    pub nodes: Vec<AnimationNodeIndex>,
    /// Entity that owns AnimationPlayer
    pub animation_player_e: Entity,
}

impl Default for Animations {
    fn default() -> Self {
        Self {
            // state: (AnimationState::StandIdle, AnimationState::StandIdle),
            current: AnimationState::StandIdle,
            requested: None,
            nodes: Vec::new(),
            animation_player_e: Entity::PLACEHOLDER,
        }
    }
}

/// State change helpers
impl Animations {
    fn request(&mut self, state: AnimationState) {
        if self.current.is_locked() {
            return;
        }

        if self.current.clip_index() != state.clip_index() {
            self.requested = Some(state);
        } else {
            self.current = state;
        }
    }

    pub fn idle(&mut self) {
        self.request(AnimationState::StandIdle);
    }

    pub fn run(&mut self, speed: f32) {
        self.request(AnimationState::Run(speed));
    }

    pub fn sprint(&mut self, speed: f32) {
        self.request(AnimationState::Sprint(speed));
    }

    pub fn crouch(&mut self, speed: f32) {
        self.request(AnimationState::Crouch(speed));
    }

    pub fn crouch_idle(&mut self) {
        self.request(AnimationState::CrouchIdle);
    }

    pub fn dash(&mut self) {
        self.request(AnimationState::Dash);
    }

    pub fn fall(&mut self) {
        self.request(AnimationState::JumpLoop);
    }

    pub fn start_jump(&mut self) {
        self.request(AnimationState::Jump);
    }
    pub fn end_jump(&mut self) {
        self.request(AnimationState::Land);
    }

    pub fn is_jumping(&self) -> bool {
        self.current.is_jumping()
    }
    pub fn is_falling(&self) -> bool {
        self.current.is_falling()
    }
}

/// The order is important here because we use it as indexes for animation node vec
#[derive(Component, Default, Reflect, Clone, Copy, PartialEq, Debug)]
#[reflect(Component)]
pub enum AnimationState {
    #[default]
    StandIdle,
    Run(f32),
    Sprint(f32),
    Jump,
    JumpLoop,
    Land,
    Crouch(f32),
    CrouchIdle,
    Dash,
}
impl AnimationState {
    pub fn clip_index(&self) -> usize {
        match self {
            AnimationState::StandIdle => 0,
            AnimationState::Run(_) => 1,
            AnimationState::Sprint(_) => 2,
            AnimationState::Jump => 3,
            AnimationState::JumpLoop => 4,
            AnimationState::Land => 5,
            AnimationState::Crouch(_) => 6,
            AnimationState::CrouchIdle => 7,
            AnimationState::Dash => 8,
        }
    }
    /// Animations that should not be interrupted by other animations
    pub fn is_locked(&self) -> bool {
        matches!(
            self,
            AnimationState::Jump | AnimationState::Land | AnimationState::Dash
        )
    }
    pub fn is_jumping(&self) -> bool {
        matches!(self, AnimationState::Jump | AnimationState::JumpLoop)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, AnimationState::Run(_))
    }
    pub fn is_falling(&self) -> bool {
        matches!(self, AnimationState::JumpLoop)
    }
}
