//! A credits screen that can be accessed from the title screen.

use crate::prelude::*;
use bevy::{ecs::spawn::SpawnIter, prelude::*, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), spawn_credits_screen);

    app.register_type::<CreditsMusic>();
    app.load_resource::<CreditsMusic>();
    app.add_systems(OnEnter(Screen::Credits), start_credits_music);
    app.add_systems(OnExit(Screen::Credits), stop_credits_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands.spawn((
        ui_root("Credits Screen"),
        StateScoped(Screen::Credits),
        children![
            header("Created by"),
            created_by(),
            header("Assets"),
            assets(),
            btn("Back", enter_title_screen),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Joe Shmoe", "Implemented alligator wrestling AI"],
        ["Jane Doe", "Made the music for the alien invasion"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Music", "CC0 time-for-fun by smnbl"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn enter_title_screen(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for CreditsMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load(AudioSources::BG_MUSIC),
            entity: None,
        }
    }
}

fn start_credits_music(
    mut commands: Commands,
    settings: Res<Settings>,
    mut credits_music: ResMut<CreditsMusic>,
) {
    let vol = settings.sound.general * settings.sound.music;
    let handle = credits_music.handle.clone();
    credits_music.entity = Some(commands.spawn(music(handle, vol)).id());
}

fn stop_credits_music(mut commands: Commands, mut credits_music: ResMut<CreditsMusic>) {
    if let Some(entity) = credits_music.entity.take() {
        commands.entity(entity).despawn();
    }
}
