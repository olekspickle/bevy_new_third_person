use crate::prelude::*;
use avian3d::{math::PI, prelude::*};
use bevy::prelude::*;
use bevy_tnua::{
    builtins::TnuaBuiltinDash, control_helpers::TnuaSimpleAirActionsCounter, prelude::*,
};
use bevy_tnua_avian3d::*;
use leafwing_input_manager::prelude::ActionState;

/// Tnua configuration is not intuitive, this is the best demo I saw:
/// https://github.com/idanarye/bevy-tnua/blob/main/demos/src/character_control_systems/platformer_control_systems.rs
pub fn movement(
    cfg: Res<Config>,
    time: Res<Time<Virtual>>,
    //touch_input: Res<Touches>,
    action: Query<&ActionState<Action>>,
    mut tnua: Query<
        (
            &mut TnuaController,
            &mut TnuaAvian3dSensorShape,
            &mut Collider,
            &mut Transform,
        ),
        (With<Player>, Without<SceneCamera>),
    >,
    mut air_counter: Query<&mut TnuaSimpleAirActionsCounter>,
    camera: Query<&Transform, With<SceneCamera>>,
) {
    let Ok((mut controller, mut avian_collider, mut collider, mut capsule)) = tnua.get_single_mut()
    else {
        return;
    };
    let mut direction = Vec3::ZERO;

    let mut speed = cfg.player.movement.speed * time.delta_secs();

    let (state, camera_transform) = (action.single(), camera.single());
    let forward = camera_transform.forward().normalize();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z);

    if state.just_pressed(&Action::Crouch) {
        // TODO: play sink animation
    }
    if state.pressed(&Action::Crouch) {
        // TODO: replace with actual crouch animation instead of just scaling
        capsule.scale.y = 0.5;
        collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
        avian_collider.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
        speed /= 2.0;
    }
    if state.just_released(&Action::Crouch) {
        // TODO: replace with actual rise animation instead of just scaling
        collider.set_scale(Vec3::ONE, 4);
        avian_collider.0.set_scale(Vec3::ONE, 4);
        capsule.scale.y = 1.0;
    }

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
    //info!(
    //    "air counter:{}",
    //    air_counter.get_count_mut().unwrap_or(&mut 0)
    //);

    if state.pressed(&Action::Jump) {
        //let jump = jump.single();
        controller.action(TnuaBuiltinJump {
            // The height is the only mandatory field of the jump button.
            height: 1.0,
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
