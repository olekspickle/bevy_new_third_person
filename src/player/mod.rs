use crate::*;
use bevy::platform::time::Instant;
use bevy_ahoy::{CharacterLook, prelude::*};
use bevy_enhanced_input::prelude::*;
use std::collections::HashMap;

mod animation;
mod control;
mod input;
mod sound;

pub use animation::*;
pub use control::*;
pub use input::*;

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        control::plugin,
        sound::plugin,
        animation::plugin,
        input::plugin,
    ))
    .add_systems(OnEnter(Screen::Gameplay), spawn_player)
    .add_observer(player_post_spawn);
}

pub fn spawn_player(
    cfg: Res<Config>,
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    #[cfg(feature = "fpv")] camera: Single<Entity, With<SceneCamera>>,
    mut commands: Commands,
    // DEBUG
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result {
    let Some(gltf) = gltf_assets.get(&models.player) else {
        return Ok(());
    };

    let mesh = SceneRoot(gltf.scenes[0].clone());
    let pos = Vec3::from(cfg.player.spawn_pos);
    let pos = Transform::from_translation(pos);
    let hitbox = Capsule3d::new(cfg.player.hitbox.radius, cfg.player.hitbox.height);
    let collider = Collider::from(hitbox);

    let _player = commands
        .spawn((
            DespawnOnExit(Screen::Gameplay),
            pos,
            Player::default(),
            // controller
            (
                PlayerInput,
                CharacterController {
                    crouch_height: 2.0,
                    gravity: cfg.physics.gravity,
                    speed: cfg.player.movement.speed(),
                    max_speed: cfg.player.movement.max_speed,
                    crouch_speed_scale: cfg.player.movement.crouch_factor,
                    jump_height: cfg.player.movement.jump_height,
                    ..default()
                },
                CharacterLook::default(),
            ),
            (
                #[cfg(feature = "third_person")]
                ThirdPersonCameraTarget,
                #[cfg(feature = "top_down")]
                TopDownCameraTarget,
            ),
            // physics
            (
                collider,
                RigidBody::Dynamic,
                Friction::default(),
                Mass(10.0),
            ),
            // other player related components
            (
                StepTimer(Timer::from_seconds(cfg.timers.step, TimerMode::Repeating)),
                InheritedVisibility::default(), // silence the warning because of adding SceneRoot as a child
            ),
        ))
        // spawn character mesh as child to adjust mesh position relative to the player origin
        .with_children(|parent| {
            let mut e = parent.spawn((mesh, Transform::from_xyz(0.0, -1.0, 0.0)));
            info!("spawning player: {}", e.id());
            e.observe(prepare_animations);

            // DEBUG
            let collider_mesh = Mesh::from(hitbox);
            let debug_collider_mesh = Mesh3d(meshes.add(collider_mesh.clone()));
            let debug_collider_color: MeshMaterial3d<StandardMaterial> =
                MeshMaterial3d(materials.add(Color::srgba(0.9, 0.1, 0.9, 0.1)));
            parent.spawn((
                debug_collider_mesh,
                debug_collider_color,
                Transform::from_xyz(0.0, -0.1, 0.0),
            ));
            // DEBUG
        })
        .id();

    #[cfg(feature = "fpv")]
    commands
        .entity(*camera)
        .insert(CharacterControllerCameraOf::new(_player));

    Ok(())
}

fn player_post_spawn(on: On<Add, Player>, mut players: Query<&mut Player>) {
    if let Ok(mut p) = players.get_mut(on.entity) {
        p.id = on.entity; // update player id with spawned entity
        info!("NEW PLAYER ID: {}", p.id);
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct Player {
    /// Will be used in the UI and split screen
    pub id: Entity,
    /// Used for time based effects, like slide dust or magic attacks
    pub last_input_change: Instant,
    pub animation: Animation,
}

/// Its fine until we have 10 mil entities on spawn I guess
const PLACEHOLDER_ENTITY: Entity = Entity::from_raw_u32(10_000_000).unwrap();

impl Default for Player {
    fn default() -> Self {
        Self {
            // FIXME: stupid hack
            // u32::MAX is Entity::PLACEHOLDER and using placeholder leads to issues and using option
            // here while being idiomatic will unnecessary complicate handling it in systems
            // We replace it with real id when the player entity is spawned anyway
            id: PLACEHOLDER_ENTITY,
            animation: Animation::default(),
            last_input_change: Instant::now(),
        }
    }
}

/// Holds all animations, their [`AnimationGraph`] node index and state of the animation
#[derive(Component, Reflect, Clone, Debug)]
pub struct Animation {
    /// Current animation state: current, next
    pub state: (AnimationState, AnimationState),
    /// Animation map,
    pub map: HashMap<String, AnimationNodeIndex>,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            state: (AnimationState::StandIdle, AnimationState::StandIdle),
            map: HashMap::new(),
        }
    }
}

impl Animation {
    pub fn next(&self) -> AnimationState {
        self.state.1
    }
    pub fn is_jumping(&self) -> bool {
        self.state.0 == AnimationState::JumpStart
            || self.state.0 == AnimationState::JumpLoop
            || self.state.0 == AnimationState::JumpLand
    }
    pub fn alter(&self) -> bool {
        self.state.0 != self.state.1
    }
    pub fn idle(&mut self) {
        self.set_if_not_already(AnimationState::StandIdle);
    }
    pub fn jump_start(&mut self) {
        self.set_if_not_already(AnimationState::JumpStart);
    }
    pub fn jump_loop(&mut self) {
        self.set_if_not_already(AnimationState::JumpLoop);
    }
    pub fn jump_land(&mut self) {
        self.set_if_not_already(AnimationState::JumpLand);
    }
    pub fn run(&mut self, speed: f32) {
        self.set_if_not_already(AnimationState::Run(speed));
    }
    pub fn sprint(&mut self, speed: f32) {
        self.set_if_not_already(AnimationState::Sprint(speed));
    }
    pub fn crouch(&mut self, speed: f32) {
        self.set_if_not_already(AnimationState::Crouch(speed));
    }
    pub fn dash(&mut self) {
        self.set_if_not_already(AnimationState::Dash);
    }
    fn set_if_not_already(&mut self, state: AnimationState) {
        if self.state.1 != state {
            self.state.1 = state;
        }
    }
}

#[derive(Component, Reflect, PartialEq, Default, Debug, Clone, Copy)]
#[reflect(Component)]
pub enum AnimationState {
    #[default]
    StandIdle,
    Run(f32),
    Sprint(f32),
    Climb(f32),
    JumpStart,
    JumpLoop,
    JumpLand,
    Fall,
    Crouch(f32),
    CrouchIdle,
    Dash,
    WallSlide,
    WallJump,
    Knockback,
}
