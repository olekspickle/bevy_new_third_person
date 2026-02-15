use super::*;

const FADE_DURATION: f32 = 0.3;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_fade_component)
        .add_systems(Update, screen_fade)
        .add_observer(on_fade_cam);
}

markers!(ScreenFadeLabel);

fn spawn_fade_component(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(Screen::Gameplay),
        ScreenFadeLabel,
        Name::new("Fade screen"),
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        GlobalZIndex(10), // ensure it’s on top
        BackgroundColor(Color::BLACK.with_alpha(0.0)),
        Pickable::IGNORE,
    ));
}

fn screen_fade(
    time: Res<Time>,
    // fade_q: Query<Entity, With<ScreenFadeLabel>>,
    // mut commands: Commands,
    mut screen_fade_q: Query<(&mut ScreenFader, &mut BackgroundColor)>,
) {
    for (mut fade, mut color) in &mut screen_fade_q {
        fade.timer.tick(time.delta());
        let t = fade.timer.fraction();

        match fade.phase {
            ScreenFadePhase::FadeOut => {
                color.0.set_alpha(t);
                if fade.timer.just_finished() {
                    fade.phase = ScreenFadePhase::Hold;
                    fade.timer = Timer::from_seconds(FADE_DURATION, TimerMode::Once);
                }
            }
            ScreenFadePhase::Hold => {
                if fade.timer.just_finished() {
                    fade.phase = ScreenFadePhase::FadeIn;
                    fade.timer = Timer::from_seconds(FADE_DURATION, TimerMode::Once);
                }
            }
            ScreenFadePhase::FadeIn => {
                color.0.set_alpha(1.0 - t);
                if fade.timer.just_finished() {
                    fade.phase = ScreenFadePhase::Finish;
                }
            }
            ScreenFadePhase::Finish => {
                // TODO: despawn the fade component or not? not sure
                // if let Ok(e) = fade_q.single() {
                //     commands.entity(e).despawn();
                // }
            }
        }
    }
}

#[derive(Event)]
pub struct FadeCam(pub ScreenFadePhase);

#[derive(Component)]
pub struct ScreenFader {
    pub phase: ScreenFadePhase,
    pub timer: Timer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScreenFadePhase {
    FadeOut,
    Hold,
    FadeIn,
    Finish,
}

fn on_fade_cam(
    on: On<FadeCam>,
    mut commands: Commands,
    fade: Single<Entity, With<ScreenFadeLabel>>,
) {
    commands.entity(*fade).insert(ScreenFader {
        phase: on.0,
        timer: Timer::from_seconds(FADE_DURATION, TimerMode::Once),
    });
}
