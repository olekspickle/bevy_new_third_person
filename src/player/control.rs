use super::*;
use bevy_ahoy::CharacterLook;
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, movement.run_if(in_state(Screen::Gameplay)));
    //     .add_observer(handle_sprint_in)
    //     .add_observer(handle_sprint_out)
    //     .add_observer(handle_jump)
    //     .add_observer(handle_dash)
    //     // .add_observer(handle_attack)
    //     .add_observer(crouch_in)
    //     .add_observer(crouch_out);
}

/// Tnua configuration is tricky to grasp from the get go, this is the best demo:
/// <https://github.com/idanarye/bevy-tnua/blob/main/demos/src/character_control_systems/platformer_control_systems.rs>
fn movement(
    cfg: Res<Config>,
    // time: Res<Time>,
    navigate: Single<&Action<Movement>>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_q: Query<
        (
            &mut Player,
            &mut Transform,
            &mut StepTimer,
            &mut CharacterLook,
            &CharacterController,
        ),
        Without<SceneCamera>,
    >,
) -> Result {
    // let dt = time.delta_secs();
    let navigate = *navigate.into_inner();

    for (mut player, mut pos, mut step_timer, mut look, ahoy) in player_q.iter_mut() {
        let cam_transform = camera.single()?;
        let input_dir = cam_transform.movement_direction(*navigate);
        let rotation = Quat::from_rotation_y(-input_dir.x.atan2(-input_dir.z));
        *look = CharacterLook::from_quat(rotation);
        pos.rotation = rotation;

        // info!(
        //     "speed: {}, animation state:{:?}",
        //     ahoy.speed,
        //     player.animation.state
        // );

        // update step timer dynamically based on actual speed
        // Note: this is specific to the animation provided
        // normal step: 0.475
        // sprint step (x1.5): 0.354
        // step on sprint timer: 0.317
        if ahoy.speed > cfg.player.movement.idle_to_run_threshold {
            let ratio = cfg.player.movement.max_speed / ahoy.speed;
            let adjusted_step_time_f32 = cfg.timers.step * ratio;
            let adjusted_step_time = Duration::from_secs_f32(adjusted_step_time_f32);
            // info!("step timer:{adjusted_step_time_f32}s");
            step_timer.set_duration(adjusted_step_time);
            player.animation.run(ahoy.speed);
        } else {
            player.animation.idle()
        }
    }

    Ok(())
}

fn handle_sprint_in(
    on: On<Start<Sprint>>,
    cfg: Res<Config>,
    mut player_q: Query<(&mut Player, &mut CharacterController)>,
) -> Result {
    let entity = on.context;
    if let Ok((mut player, mut ahoy)) = player_q.get_mut(entity) {
        if ahoy.speed <= cfg.player.movement.max_speed {
            ahoy.speed *= cfg.player.movement.sprint_factor;
            player.animation.sprint(ahoy.speed);
            info!("Sprint started for entity: {entity}");
        }
    }

    Ok(())
}

fn handle_sprint_out(
    on: On<Complete<Movement>>,
    cfg: Res<Config>,
    mut player_q: Query<(&mut Player, &mut CharacterController)>,
) {
    if let Ok((mut player, mut ahoy)) = player_q.get_mut(on.context) {
        if ahoy.speed > cfg.player.movement.max_speed {
            player.animation.state.1 = AnimationState::Run(ahoy.speed);
        }
    }
}

fn handle_jump(
    on: On<Fire<Jump>>,
    // cfg: Res<Config>,
    // time: Res<Time>,
    mut player_query: Query<(&mut Player, &CharacterController, &mut JumpTimer), With<Player>>,
) -> Result {
    let (mut player, ahoy, mut _jump_timer) = player_query.get_mut(on.context)?;

    info!("jumping with speed: {}", ahoy.speed);

    // if jump_timer.tick(time.delta()).just_finished() {
    // }

    Ok(())
}

fn handle_dash(
    on: On<Start<Dash>>,
    navigate: Single<&Action<Movement>>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_q: Query<&mut Player>,
) -> Result {
    let player = player_q.get_mut(on.context)?;
    let cam_transform = camera.single()?;
    let navigate = **navigate.into_inner();
    let direction = cam_transform.movement_direction(navigate);

    // TODO: dash

    Ok(())
}

// fn handle_attack(on: On<Start<Attack>>, mut commands: Commands) {
//     let entity = on.target();
//     // TODO: Hit
// }

pub fn crouch_in(
    on: On<Start<Crouch>>,
    mut player: Query<(&mut Player, &mut Transform), With<PlayerCtx>>,
    mut collider: Query<&mut Collider, With<Player>>,
) -> Result {
    let mut collider = collider.single_mut()?;
    let (mut player, mut transform) = player.get_mut(on.context)?;

    transform.scale = Vec3::new(1.0, 0.5, 1.0);
    collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    // avian_sensor.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    // ahoy.speed *= cfg.player.movement.crouch_factor;

    // TODO: Handle slide

    Ok(())
}

pub fn crouch_out(
    on: On<Complete<Crouch>>,
    mut player: Query<(&mut Player, &mut Transform), With<PlayerCtx>>,
    mut collider: Query<&mut Collider, (With<Player>, Without<SceneCamera>)>,
) -> Result {
    let mut collider = collider.get_mut(on.context)?;
    let (mut player, mut transform) = player.get_mut(on.context)?;

    collider.set_scale(Vec3::ONE, 4);
    transform.scale = Vec3::new(1.0, 1.0, 1.0);
    // avian_sensor.0.set_scale(Vec3::ONE, 4);

    // TODO: Handle slide

    Ok(())
}
