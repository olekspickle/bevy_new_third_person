use super::*;
use bevy_ahoy::CharacterLook;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        movement
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::UserInput),
    )
    .add_observer(handle_sprint_in)
    .add_observer(handle_sprint_out)
    .add_observer(handle_dash)
    //  .add_observer(handle_jump)
    //  .add_observer(handle_attack)
    .add_observer(crouch_in)
    .add_observer(crouch_out);
}

#[derive(Component)]
pub struct Dashing(pub Instant);

/// Tnua configuration is tricky to grasp from the get go, this is the best demo:
/// <https://github.com/idanarye/bevy-tnua/blob/main/demos/src/character_control_systems/platformer_control_systems.rs>
fn movement(
    movement: Single<&Action<Movement>>,
    camera: Single<&Transform, With<SceneCamera>>,
    mut player_q: Query<(&Player, &mut Transform, &mut CharacterLook), Without<SceneCamera>>,
) -> Result {
    let movement = *movement.into_inner();

    for (_p, mut pos, mut look) in player_q.iter_mut() {
        let input_dir = camera.movement_direction(*movement);

        if input_dir.length_squared() > 0.0 {
            // set ahoy KCC direction
            let (yaw, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
            *look = CharacterLook { yaw, pitch };
            // rotate model
            let rotation = Quat::from_rotation_y(input_dir.x.atan2(input_dir.z));
            pos.rotation = pos.rotation.slerp(rotation, 0.2);
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
        if ahoy.speed == cfg.player.movement.speed() {
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
        if ahoy.speed > cfg.player.movement.speed() {
            player.animation.state.1 = AnimationState::Run(ahoy.speed);
            ahoy.speed = cfg.player.movement.speed();
        }
    }
}

fn handle_dash(
    on: On<Start<Dash>>,
    navigate: Single<&Action<Movement>>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_q: Query<&mut Player>,
    mut commands: Commands,
) -> Result {
    let player = player_q.get_mut(on.context)?;
    commands.entity(player.id).insert(Dashing(Instant::now()));

    let cam_transform = camera.single()?;
    let navigate = **navigate.into_inner();
    let _direction = cam_transform.movement_direction(navigate);

    // TODO: dash

    Ok(())
}

pub fn crouch_in(
    on: On<Start<Crouch>>,
    mut player: Query<(
        &mut Player,
        &mut Collider,
        &mut Transform,
        &CharacterController,
    )>,
) -> Result {
    let (mut player, mut collider, mut transform, ahoy) = player.get_mut(on.context)?;

    transform.scale = Vec3::new(1.0, 0.5, 1.0);
    collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    player.animation.state.1 = AnimationState::Crouch(ahoy.speed);

    // TODO: Handle slide

    Ok(())
}

pub fn crouch_out(
    on: On<Complete<Crouch>>,
    mut player: Query<(
        &mut Player,
        &mut Collider,
        &mut Transform,
        &CharacterController,
    )>,
) -> Result {
    let (mut player, mut collider, mut transform, ahoy) = player.get_mut(on.context)?;

    transform.scale = Vec3::new(1.0, 1.0, 1.0);
    collider.set_scale(Vec3::ONE, 4);
    player.animation.state.1 = AnimationState::Run(ahoy.speed);

    // TODO: Handle slide

    Ok(())
}

// fn handle_attack(on: On<Start<Attack>>, mut commands: Commands) {
//     let entity = on.target();
//     // TODO: Hit
// }

// fn handle_jump(
//     on: On<Fire<Jump>>,
//     // cfg: Res<Config>,
//     // time: Res<Time>,
//     mut player_query: Query<(&mut Player, &CharacterController, &mut JumpTimer), With<Player>>,
// ) -> Result {
//     let (mut player, ahoy, mut _jump_timer) = player_query.get_mut(on.event_target())?;
//
//     info!("jumping with speed: {}", ahoy.speed);
//
//     // if jump_timer.tick(time.delta()).just_finished() {
//     // }
//
//     Ok(())
// }
