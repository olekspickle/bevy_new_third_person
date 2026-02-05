use super::*;

pub fn plugin(app: &mut App) {
    app.add_observer(movement_sound)
        .add_observer(dash_sound)
        .add_observer(jump_sound);
}

#[allow(clippy::too_many_arguments)]
fn movement_sound(
    _on: On<Fire<Movement>>,
    state: Res<GameState>,
    // time: Res<Time>,
    // settings: Res<Settings>,
    // crouch: Single<&Action<Crouch>>,
    // mut commands: Commands,
    // mut sources: ResMut<AudioSources>,
    // mut step_timer: Query<&mut StepTimer, With<Player>>,
) -> Result {
    if state.muted || state.paused {
        return Ok(());
    }

    // WALK SOUND
    // let mut step_timer = step_timer.get_mut(on.context)?;
    // if step_timer.tick(time.delta()).just_finished() && basis.standing_on_entity().is_some() {
    //     let mut rng = rand::rng();
    //     let crouch = ***crouch;
    //     let handle = if crouch {
    //         // TODO: select crouch steps
    //         sources.steps.pick(&mut rng)
    //     } else {
    //         sources.steps.pick(&mut rng)
    //     };
    //     commands.spawn(SamplePlayer::new(handle.clone()).with_volume(settings.sfx()));
    // }

    Ok(())
}

fn jump_sound(
    _: On<Start<Jump>>,
    state: Res<GameState>,
    settings: Res<Settings>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
) -> Result {
    if state.muted || state.paused {
        return Ok(());
    }

    // let jump_timer = jump_timer.get(on.target())?;
    // if jump_timer.just_finished() {
    let mut rng = rand::rng();
    let handle = sources.steps.pick(&mut rng);
    commands.spawn(SamplePlayer::new(handle.clone()).with_volume(settings.sfx()));
    // }

    Ok(())
}

fn dash_sound(
    _: On<Start<Dash>>,
    state: Res<GameState>,
    settings: Res<Settings>,
    mut commands: Commands,
    mut sources: ResMut<AudioSources>,
) -> Result {
    if state.muted || state.paused {
        return Ok(());
    }

    let mut rng = rand::rng();
    let handle = sources.steps.pick(&mut rng);
    commands.spawn(SamplePlayer::new(handle.clone()).with_volume(settings.sfx()));

    Ok(())
}
