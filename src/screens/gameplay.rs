//! The screen state for the main gameplay.

use super::*;
use bevy::ui::Val::*;
use bevy_third_person_camera::ThirdPersonCamera;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(crate::game::plugin)
        .add_event::<OnMenuToggle>()
        .add_systems(OnEnter(Screen::Gameplay), spawn_gameplay_ui)
        .add_systems(
            Update,
            (toot, invoke_menu)
                .run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_observer(toggle_menu);
}

#[derive(Component)]
pub struct DevTools;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;
// TODO: The idea is to create a boombox with spatial audio
// <https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_3d.rs>
// #[derive(Component)]
// pub struct Boombox;

fn spawn_gameplay_ui(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DevTools,
        Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![
            // Demo keys
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
                children![
                    (label("P - pause"), PauseLabel),
                    (label("M - mute"), MuteLabel),
                    label("F - diagnostics"),
                    label("V - incr fov"),
                    (
                        label(format!("O - toggle sun cycle: {:?}", settings.sun_cycle)),
                        SunCycleLabel
                    ),
                    TextLayout::new_with_justify(JustifyText::Left)
                ]
            ),
        ],
    ));
}

fn toot(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Toot) {
        let vol = settings.sound.general * settings.sound.sfx;
        commands.spawn(sfx(sources.btn_press.clone(), vol));
    }

    Ok(())
}

#[derive(Event)]
pub struct OnMenuToggle;
#[derive(Component)]
pub struct Menu;

fn invoke_menu(mut commands: Commands, action: Query<&ActionState<Action>>) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Menu) {
        commands.trigger(OnMenuToggle);
    }

    Ok(())
}

fn trigger_toggle_menu(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnMenuToggle);
}

fn toggle_menu(
    _: Trigger<OnMenuToggle>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    menu_label: Query<Entity, With<Menu>>,
    mut cam: Query<&mut ThirdPersonCamera>,
) {
    let Ok(mut cam) = cam.single_mut() else {
        return;
    };
    cam.cursor_lock_active = !cam.cursor_lock_active;
    settings.menu_open = !settings.menu_open;

    if settings.menu_open {
        commands.spawn(menu_modal());
    } else if let Ok(menu) = menu_label.single() {
        commands.entity(menu).despawn();
    }
}

fn menu_modal() -> impl Bundle {
    let opts = Opts::new("x");
    (
        Menu,
        ui_root("In game menu"),
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                width: Percent(20.0),
                height: Percent(30.0),
                ..Default::default()
            },
            BackgroundColor(TRANSLUCENT),
            BorderRadius::all(Px(BORDER_RADIUS)),
            children![
                Node::default(),
                btn_small(opts.clone(), trigger_toggle_menu),
                btn(opts.clone().text("Settings"), gameplay_to_settings),
                btn(opts.text("Main Menu"), gameplay_to_title)
            ]
        )],
    )
}

pub fn gameplay_to_title(
    _trigger: Trigger<OnPress>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    commands.trigger(OnMenuToggle);
    next_screen.set(Screen::Title);
}

pub fn gameplay_to_settings(
    _trigger: Trigger<OnPress>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    commands.trigger(OnMenuToggle);
    next_screen.set(Screen::Settings);
}
