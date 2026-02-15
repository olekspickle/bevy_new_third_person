use super::*;
use crate::{
    player::{Player, modal_ctx_active, player_ctx_active},
    scene::ScreenFadePhase,
};
use bevy_yarnspinner::{
    events::{DialogueCompleted, DialogueStarted},
    prelude::*,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        spawn_start_dialogue.after(crate::player::spawn_player),
    )
    .add_systems(
        Update,
        start_cutscene.run_if(any_with_component::<DialogueRunner>),
    )
    .add_observer(on_dialogue_start)
    .add_observer(on_dialogue_complete);
}

fn spawn_start_dialogue(
    mut commands: Commands,
    project: Res<YarnProject>,
    player_q: Query<Entity, With<Player>>,
) {
    for player in player_q.iter() {
        let mut dialogue_runner = project.create_dialogue_runner(&mut commands);
        dialogue_runner.start_node("Start");
        commands.entity(player).insert(dialogue_runner);
    }
}

fn start_cutscene(mut commands: Commands, mut dialogue_runner_q: Query<&mut DialogueRunner>) {
    for mut runner in dialogue_runner_q.iter_mut() {
        let storage = runner.variable_storage_mut();
        let fade = storage.get("$fade");
        let fade = fade
            .map(|v| bool::try_from(v).ok())
            .unwrap_or_default()
            .unwrap_or_default();

        if fade {
            debug!("Fading out screen: {:?}", fade);
            commands.trigger(FadeCam(ScreenFadePhase::FadeOut));
            storage
                .set("$fade".to_string(), false.into())
                .unwrap_or_default();
        }
    }
}

fn on_dialogue_start(on: On<DialogueStarted>, mut commands: Commands) {
    debug!("dialog started: {:?}", on.entity);
    commands
        .entity(on.event_target())
        .insert(modal_ctx_active());
}
fn on_dialogue_complete(on: On<DialogueCompleted>, mut commands: Commands) {
    debug!("dialog completed: {:?}", on.entity);
    commands
        .entity(on.event_target())
        .insert(player_ctx_active())
        .trigger(ToggleCamCursor);
}
