use crate::prelude::*;
use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_tnua::{TnuaAnimatingState, control_helpers::TnuaSimpleAirActionsCounter, prelude::*};
use bevy_tnua_avian3d::*;

mod animation;
mod control;

pub use animation::*;
pub use control::*;

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        ThirdPersonCameraPlugin,
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));

    app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync))
        .add_systems(OnEnter(Screen::Gameplay), spawn)
        .add_systems(
            Update,
            (movement, animating)
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Component)]
pub struct Player {
    speed: f32,
    animation_state: AnimationState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 1.0,
            animation_state: AnimationState::StandIdle,
        }
    }
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
    let player = Player {
        speed: cfg.player.movement.speed,
        animation_state: AnimationState::StandIdle,
    };

    let collider = Collider::capsule(cfg.player.hitbox.radius, cfg.player.hitbox.height);
    let collider_mesh = Mesh::from(Capsule3d::new(
        cfg.player.hitbox.radius,
        cfg.player.hitbox.height,
    ));
    let debug_collider_mesh = Mesh3d(meshes.add(collider_mesh.clone()));
    let debug_collider_color: MeshMaterial3d<StandardMaterial> =
        MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.2)));

    commands
        .spawn((
            pos,
            player,
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
            JumpTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
            StepTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
            debug_collider_mesh,
            debug_collider_color,
        ))
        .with_children(|children| {
            ChildBuild::spawn(children, (Transform::from_xyz(0.0, -1.0, 0.0), mesh))
                .observe(prepare_animations);
        });
}
